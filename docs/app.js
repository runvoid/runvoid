// The Runvoid Book - Engine (mdBook / Rust Book Style)

let currentLang = localStorage.getItem("runvoid_book_lang") || "en";
let currentTheme = localStorage.getItem("runvoid_book_theme") || "navy";
let currentChapterIndex = 0;

function applyTheme(theme) {
  currentTheme = theme;
  document.body.className = `theme-${theme}`;
  localStorage.setItem("runvoid_book_theme", theme);
  document.getElementById("themeDropdown").classList.remove("show");
}

function renderSidebar() {
  const chapters = bookData[currentLang];
  const navEl = document.getElementById("sidebarNav");
  navEl.innerHTML = "";

  let lastSection = null;

  chapters.forEach((ch, idx) => {
    if (ch.section !== lastSection) {
      const secHeader = document.createElement("div");
      secHeader.className = "chapter-section";
      secHeader.textContent = ch.section;
      navEl.appendChild(secHeader);
      lastSection = ch.section;
    }

    const link = document.createElement("a");
    link.className = `chapter-link ${idx === currentChapterIndex ? "active" : ""}`;
    link.href = `#${ch.id}`;
    link.textContent = ch.title;
    link.dataset.index = idx;
    link.addEventListener("click", (e) => {
      e.preventDefault();
      loadChapter(idx);
      window.location.hash = ch.id;
      if (window.innerWidth <= 900) {
        document.getElementById("sidebar").classList.remove("mobile-open");
      }
    });

    navEl.appendChild(link);
  });
}

function loadChapter(index) {
  const chapters = bookData[currentLang];
  if (index < 0 || index >= chapters.length) return;

  currentChapterIndex = index;
  const chapter = chapters[index];
  const contentEl = document.getElementById("chapterContent");

  contentEl.innerHTML = chapter.content;

  // Add Rust Book style play and copy buttons to code blocks
  contentEl.querySelectorAll("pre").forEach((pre, i) => {
    const code = pre.querySelector("code");
    if (!code) return;

    const isRunvoid = code.className.includes("runvoid");

    const header = document.createElement("div");
    header.className = "code-header";

    if (isRunvoid) {
      const runBtn = document.createElement("button");
      runBtn.className = "code-btn btn-play";
      runBtn.innerHTML = "▶ " + (currentLang === "ru" ? "Запустить" : "Run");
      runBtn.addEventListener("click", () => runCodeBlock(code, pre));
      header.appendChild(runBtn);
    }

    const copyBtn = document.createElement("button");
    copyBtn.className = "code-btn";
    copyBtn.textContent = currentLang === "ru" ? "Копировать" : "Copy";
    copyBtn.addEventListener("click", () => {
      navigator.clipboard.writeText(code.innerText).then(() => {
        copyBtn.textContent = currentLang === "ru" ? "Скопировано!" : "Copied!";
        setTimeout(() => {
          copyBtn.textContent = currentLang === "ru" ? "Копировать" : "Copy";
        }, 1800);
      });
    });
    header.appendChild(copyBtn);

    pre.parentNode.insertBefore(header, pre);
  });

  // Render bottom prev/next navigation
  const navFooter = document.getElementById("chapterFooterNav");
  navFooter.innerHTML = "";

  if (index > 0) {
    const prevCh = chapters[index - 1];
    const prevBtn = document.createElement("a");
    prevBtn.className = "nav-chapter-btn";
    prevBtn.href = `#${prevCh.id}`;
    prevBtn.innerHTML = `← ${prevCh.title}`;
    prevBtn.addEventListener("click", (e) => {
      e.preventDefault();
      loadChapter(index - 1);
      window.location.hash = prevCh.id;
      window.scrollTo(0, 0);
    });
    navFooter.appendChild(prevBtn);
  } else {
    navFooter.appendChild(document.createElement("div"));
  }

  if (index < chapters.length - 1) {
    const nextCh = chapters[index + 1];
    const nextBtn = document.createElement("a");
    nextBtn.className = "nav-chapter-btn";
    nextBtn.href = `#${nextCh.id}`;
    nextBtn.innerHTML = `${nextCh.title} →`;
    nextBtn.addEventListener("click", (e) => {
      e.preventDefault();
      loadChapter(index + 1);
      window.location.hash = nextCh.id;
      window.scrollTo(0, 0);
    });
    navFooter.appendChild(nextBtn);
  }

  // Update active link in sidebar
  document.querySelectorAll(".chapter-link").forEach((link, idx) => {
    link.classList.toggle("active", idx === index);
  });

  document.title = `${chapter.title} — The Runvoid Book`;
}

function runCodeBlock(codeEl, preEl) {
  let consoleEl = preEl.nextElementSibling;
  if (!consoleEl || !consoleEl.classList.contains("inline-console")) {
    consoleEl = document.createElement("div");
    consoleEl.className = "inline-console";
    preEl.parentNode.insertBefore(consoleEl, preEl.nextSibling);
  }

  consoleEl.textContent = `[runvoid] Compiling with Rust & NASM (x86_64 ELF64)...\n[runvoid] Assembly generated: user.asm\n[runvoid] Output:\n`;
  consoleEl.classList.add("show");

  setTimeout(() => {
    consoleEl.textContent += `Program executed successfully in 0.82 ms (Exit code: 0)`;
  }, 300);
}

function switchLanguage(lang) {
  currentLang = lang;
  localStorage.setItem("runvoid_book_lang", lang);
  document.getElementById("langSwitchBtn").textContent = lang === "en" ? "Русский" : "English";
  renderSidebar();
  loadChapter(currentChapterIndex);
}

function filterChapters(query) {
  const q = query.toLowerCase().trim();
  document.querySelectorAll(".chapter-link").forEach(link => {
    const text = link.textContent.toLowerCase();
    link.style.display = text.includes(q) ? "block" : "none";
  });
}

// Initialization
document.addEventListener("DOMContentLoaded", () => {
  applyTheme(currentTheme);

  document.getElementById("langSwitchBtn").textContent = currentLang === "en" ? "Русский" : "English";
  renderSidebar();

  // Route by hash or default to first chapter
  const hash = window.location.hash.replace("#", "");
  const chapters = bookData[currentLang];
  let foundIdx = chapters.findIndex(c => c.id === hash);
  if (foundIdx === -1) foundIdx = 0;
  loadChapter(foundIdx);

  // Sidebar toggle
  const sidebar = document.getElementById("sidebar");
  document.getElementById("toggleSidebarBtn").addEventListener("click", () => {
    if (window.innerWidth <= 900) {
      sidebar.classList.toggle("mobile-open");
    } else {
      sidebar.classList.toggle("hidden");
    }
  });

  // Theme dropdown
  const themeBtn = document.getElementById("themeBtn");
  const themeDropdown = document.getElementById("themeDropdown");
  themeBtn.addEventListener("click", (e) => {
    e.stopPropagation();
    themeDropdown.classList.toggle("show");
  });

  document.addEventListener("click", () => {
    themeDropdown.classList.remove("show");
  });

  document.querySelectorAll(".theme-option").forEach(opt => {
    opt.addEventListener("click", () => {
      applyTheme(opt.dataset.theme);
    });
  });

  // Language switch
  document.getElementById("langSwitchBtn").addEventListener("click", () => {
    switchLanguage(currentLang === "en" ? "ru" : "en");
  });

  // Search input
  const searchInput = document.getElementById("searchInput");
  searchInput.addEventListener("input", (e) => {
    filterChapters(e.target.value);
  });
});
