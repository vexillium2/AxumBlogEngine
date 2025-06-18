use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::database::orm::get_conn;
use crate::error::CustomResult;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, Validate)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[serde(skip_serializing)]
    pub password_hash: String,
    
    pub role: String,
    
    #[serde(with = "datetime_serializer")]
    pub created_at: NaiveDateTime,
    
    #[serde(with = "datetime_serializer")]
    pub updated_at: NaiveDateTime,
}

// 时间序列化模块
mod datetime_serializer {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}

pub async fn create(
    username: &str,
    email: &str,
    password_hash: &str,
    role: Option<&str>,
) -> CustomResult<Model> {
    let conn = get_conn().await;
    let model = ActiveModel {
        username: Set(username.to_owned()),
        email: Set(email.to_owned()),
        password_hash: Set(password_hash.to_owned()),
        role: Set(role.unwrap_or("user").to_owned()),
        ..Default::default()
    };
    
    Ok(model.insert(conn).await?)
}

pub async fn find_by_username(username: &str) -> CustomResult<Option<Model>> {
    let conn = get_conn().await;
    Entity::find()
        .filter(Column::Username.eq(username))
        .one(conn)
        .await
        .map_err(Into::into)
}

pub async fn find_by_email(email: &str) -> CustomResult<Option<Model>> {
    let conn = get_conn().await;
    Entity::find()
        .filter(Column::Email.eq(email))
        .one(conn)
        .await
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::{Fake, Faker};

    async fn create_test_user() -> Model {
        create(
            &Faker.fake::<String>(),
            &Faker.fake::<String>(),
            "test_hash",
            None,
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn test_user_creation() {
        dotenv::dotenv().ok();
        let user = create_test_user().await;
        assert!(!user.is_admin());
    }

    #[tokio::test]
    async fn test_find_user() {
        dotenv::dotenv().ok();
        let test_user = create_test_user().await;
        
        let found = find_by_username(&test_user.username)
            .await
            .unwrap()
            .expect("User not found");
            
        assert_eq!(found.id, test_user.id);
    }
}