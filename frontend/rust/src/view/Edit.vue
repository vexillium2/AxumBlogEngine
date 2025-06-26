<template>
  <div class="editor-container">
    <div class="editor-header">
      <h2>{{ mode === "create" ? "新建文章" : "编辑文章" }}</h2>
      <div class="header-actions">
        <button class="btn btn-secondary" @click="cancel">取消</button>
        <button class="btn btn-primary" @click="saveDraft">保存草稿</button>
        <button class="btn btn-success" @click="publish">发布</button>
      </div>
    </div>

    <div class="editor-body">
      <div class="form-group">
        <input
          type="text"
          class="title-input"
          v-model="article.title"
          placeholder="请输入文章标题..."
        />
      </div>

      <div class="form-group">
        <div class="tag-container">
          <span class="tag" v-for="(tag, index) in article.tags" :key="index">
            {{ tag }}
            <span class="tag-remove" @click="removeTag(index)">×</span>
          </span>
          <input
            type="text"
            class="tag-input"
            v-model="tagInput"
            placeholder="添加标签..."
            @keydown.enter="addTag"
            @keydown.backspace="handleBackspace"
          />
        </div>
      </div>

      <div class="form-group">
        <div class="toolbar">
          <button
            v-for="(item, index) in toolbarItems"
            :key="index"
            class="toolbar-btn"
            :title="item.title"
            @click="execCommand(item.command)"
          >
            <i :class="item.icon"></i>
          </button>
          <select class="heading-select" @change="formatHeading">
            <option value="">正文</option>
            <option value="h1">标题1</option>
            <option value="h2">标题2</option>
            <option value="h3">标题3</option>
          </select>
        </div>
        <div
          class="editor-content"
          ref="editor"
          contenteditable="true"
          @input="onContentChange"
          @paste="handlePaste"
        ></div>
      </div>

      <div class="form-group">
        <label class="cover-upload">
          <input
            type="file"
            accept="image/*"
            @change="uploadCover"
            style="display: none"
          />
          <span v-if="!article.cover" class="upload-text">
            <i class="icon-upload"></i> 上传封面图
          </span>
          <div v-else class="cover-preview">
            <img :src="article.cover" alt="封面预览" />
            <button class="btn btn-danger btn-sm" @click="removeCover">
              移除
            </button>
          </div>
        </label>
      </div>

      <div class="form-group">
        <label class="summary-label">文章摘要</label>
        <textarea
          class="summary-input"
          v-model="article.summary"
          placeholder="请输入文章摘要..."
          rows="3"
        ></textarea>
      </div>

      <div class="form-group">
        <label class="checkbox-label">
          <input type="checkbox" v-model="article.commentable" /> 允许评论
        </label>
        <label class="checkbox-label">
          <input type="checkbox" v-model="article.recommended" /> 推荐到首页
        </label>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, nextTick } from "vue";

const props = defineProps({
  mode: {
    type: String,
    default: "create", // 'create' or 'edit'
  },
  postData: {
    type: Object,
    default: null,
  },
});

const emit = defineEmits(["save", "cancel"]);

const editor = ref(null);
const tagInput = ref("");

const article = reactive({
  id: null,
  title: "",
  content: "",
  summary: "",
  tags: [],
  cover: "",
  published: false,
  commentable: true,
  recommended: false,
});

const toolbarItems = [
  { title: "加粗", icon: "icon-bold", command: "bold" },
  { title: "斜体", icon: "icon-italic", command: "italic" },
  { title: "下划线", icon: "icon-underline", command: "underline" },
  { title: "删除线", icon: "icon-strikethrough", command: "strikeThrough" },
  { title: "有序列表", icon: "icon-list-ol", command: "insertOrderedList" },
  { title: "无序列表", icon: "icon-list-ul", command: "insertUnorderedList" },
  { title: "链接", icon: "icon-link", command: "createLink" },
  { title: "图片", icon: "icon-image", command: "insertImage" },
  { title: "代码块", icon: "icon-code", command: "formatBlock", value: "pre" },
  {
    title: "引用",
    icon: "icon-quote-right",
    command: "formatBlock",
    value: "blockquote",
  },
  { title: "清除格式", icon: "icon-eraser", command: "removeFormat" },
];

// 初始化编辑器内容
onMounted(() => {
  if (props.mode === "edit" && props.postData) {
    Object.assign(article, props.postData);
    nextTick(() => {
      if (editor.value) {
        editor.value.innerHTML = article.content;
      }
    });
  }
});

const onContentChange = () => {
  if (editor.value) {
    article.content = editor.value.innerHTML;
  }
};

const execCommand = (command, value = null) => {
  document.execCommand(command, false, value);
  if (editor.value) {
    editor.value.focus();
  }
};

const formatHeading = (e) => {
  const value = e.target.value;
  if (value) {
    document.execCommand("formatBlock", false, value);
  }
  e.target.value = "";
};

const addTag = () => {
  const tag = tagInput.value.trim();
  if (tag && !article.tags.includes(tag)) {
    article.tags.push(tag);
    tagInput.value = "";
  }
};

const removeTag = (index) => {
  article.tags.splice(index, 1);
};

const handleBackspace = (e) => {
  if (tagInput.value === "" && article.tags.length > 0) {
    article.tags.pop();
  }
};

const uploadCover = (e) => {
  const file = e.target.files[0];
  if (file) {
    // 这里应该是实际上传逻辑，暂时使用本地URL
    const reader = new FileReader();
    reader.onload = (event) => {
      article.cover = event.target.result;
    };
    reader.readAsDataURL(file);
  }
};

const removeCover = () => {
  article.cover = "";
};

const saveDraft = () => {
  article.published = false;
  saveArticle();
};

const publish = () => {
  article.published = true;
  saveArticle();
};

const saveArticle = () => {
  if (!article.title.trim()) {
    alert("请输入文章标题");
    return;
  }

  if (!article.content.trim()) {
    alert("请输入文章内容");
    return;
  }

  // 如果是编辑模式且没有ID，则生成一个临时ID
  if (props.mode === "create" && !article.id) {
    article.id = Date.now();
  }

  emit("save", { ...article });
};

const cancel = () => {
  emit("cancel");
};

const handlePaste = (e) => {
  e.preventDefault();
  const text = (e.clipboardData || window.clipboardData).getData("text/plain");
  document.execCommand("insertText", false, text);
};
</script>

<style scoped>
.editor-container {
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.05);
  overflow: hidden;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 20px;
  border-bottom: 1px solid #f0f0f0;
}

.header-actions {
  display: flex;
  gap: 10px;
}

.editor-body {
  padding: 20px;
}

.form-group {
  margin-bottom: 20px;
}

.title-input {
  width: 100%;
  padding: 12px 15px;
  font-size: 1.5rem;
  border: none;
  border-bottom: 1px solid #f0f0f0;
  outline: none;
}

.title-input:focus {
  border-bottom-color: #1890ff;
}

.tag-container {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  align-items: center;
  padding: 8px;
  border: 1px solid #f0f0f0;
  border-radius: 4px;
}

.tag {
  background-color: #f0f0f0;
  padding: 4px 8px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  font-size: 0.9rem;
}

.tag-remove {
  margin-left: 5px;
  cursor: pointer;
  color: #999;
}

.tag-remove:hover {
  color: #ff4d4f;
}

.tag-input {
  border: none;
  outline: none;
  padding: 4px;
  flex-grow: 1;
  min-width: 100px;
}

.toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 5px;
  padding: 8px;
  border: 1px solid #f0f0f0;
  border-bottom: none;
  border-radius: 4px 4px 0 0;
}

.toolbar-btn {
  background: none;
  border: none;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border-radius: 4px;
}

.toolbar-btn:hover {
  background-color: #f0f0f0;
}

.heading-select {
  height: 32px;
  padding: 0 8px;
  border: 1px solid #f0f0f0;
  border-radius: 4px;
  outline: none;
}

.editor-content {
  min-height: 500px;
  padding: 15px;
  border: 1px solid #f0f0f0;
  border-radius: 0 0 4px 4px;
  outline: none;
  line-height: 1.6;
}

.editor-content:focus {
  border-color: #1890ff;
}

.cover-upload {
  display: block;
  cursor: pointer;
}

.upload-text {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 8px 15px;
  border: 1px dashed #d9d9d9;
  border-radius: 4px;
}

.upload-text:hover {
  border-color: #1890ff;
  color: #1890ff;
}

.cover-preview {
  position: relative;
  max-width: 300px;
}

.cover-preview img {
  width: 100%;
  border-radius: 4px;
}

.cover-preview button {
  position: absolute;
  bottom: 10px;
  right: 10px;
}

.summary-label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
}

.summary-input {
  width: 100%;
  padding: 10px;
  border: 1px solid #f0f0f0;
  border-radius: 4px;
  resize: vertical;
  outline: none;
}

.summary-input:focus {
  border-color: #1890ff;
}

.checkbox-label {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  margin-right: 15px;
  cursor: pointer;
}

.icon-bold:before {
  content: "B";
  font-weight: bold;
}

.icon-italic:before {
  content: "I";
  font-style: italic;
}

.icon-underline:before {
  content: "U";
  text-decoration: underline;
}

.icon-strikethrough:before {
  content: "S";
  text-decoration: line-through;
}

.icon-list-ol:before {
  content: "1.";
}

.icon-list-ul:before {
  content: "•";
}

.icon-link:before {
  content: "🔗";
}

.icon-image:before {
  content: "🖼️";
}

.icon-code:before {
  content: "</>";
}

.icon-quote-right:before {
  content: "❝";
}

.icon-eraser:before {
  content: "✕";
}

.icon-upload:before {
  content: "↑";
}
</style>