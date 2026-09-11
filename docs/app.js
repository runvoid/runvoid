// Runvoid GitHub Pages Interactive Engine

const snippets = {
  beginner: {
    code: `<span class="cmt">// Conversational Beginner Mode</span>
<span class="kw">say</span> <span class="str">"Welcome, adventurer!"</span>

<span class="kw">remember</span> inventory = <span class="str">"Torch"</span>, <span class="str">"Healing Potion"</span>, <span class="str">"Iron Sword"</span>
<span class="kw">add</span> <span class="str">"Magic Ring"</span> <span class="kw">to</span> inventory

<span class="kw">if</span> inventory <span class="kw">has</span> <span class="str">"Iron Sword"</span> {
    <span class="kw">say</span> green <span class="str">"Equipped and ready for battle!"</span>
}

<span class="kw">say</span> <span class="str">"Total items in pack: {count inventory}"</span>

<span class="kw">for</span> <span class="kw">every</span> item <span class="kw">in</span> inventory {
    <span class="kw">say</span> <span class="str">" - {item}"</span>
}`,
    output: `[runvoid] Compiling hello.rv -> x86_64 NASM -> ELF64...
[runvoid] Linking standalone binary (19.4 KB)...
[runvoid] Executing:

Welcome, adventurer!
\x1b[32mEquipped and ready for battle!\x1b[0m
Total items in pack: 4
 - Torch
 - Healing Potion
 - Iron Sword
 - Magic Ring`
  },

  canvas: {
    code: `<span class="cmt">// 2D Hardware Canvas & Window GUI</span>
<span class="kw">screen</span> <span class="str">"Arcade 2D"</span>, <span class="num">640</span>, <span class="num">480</span> {
    <span class="kw">draw</span> box <span class="kw">at</span> <span class="num">0</span>, <span class="num">0</span>, size <span class="num">640</span>, <span class="num">480</span>, color <span class="str">"black"</span>
    <span class="kw">draw</span> circle <span class="kw">at</span> <span class="num">320</span>, <span class="num">240</span>, size <span class="num">50</span>, color <span class="str">"cyan"</span>
    <span class="kw">draw</span> line from <span class="num">0</span>, <span class="num">0</span> <span class="kw">to</span> <span class="num">640</span>, <span class="num">480</span>, color <span class="str">"red"</span>
    <span class="kw">draw</span> text <span class="str">"Player 1 Ready"</span>, <span class="kw">at</span> <span class="num">230</span>, <span class="num">60</span>, color <span class="str">"yellow"</span>
}

<span class="kw">window</span> <span class="str">"Runvoid Panel"</span>, <span class="num">400</span>, <span class="num">250</span> {
    <span class="kw">label</span> <span class="str">"Settings:"</span>
    <span class="kw">checkbox</span> <span class="str">"Sound FX"</span>, <span class="num">1</span>
    <span class="kw">button</span> <span class="str">"Launch"</span> {
        <span class="kw">say</span> <span class="str">"Engine started!"</span>
    }
}`,
    output: `[runvoid] Compiling canvas.rv -> x86_64 NASM...
[runvoid] Linking with libX11...
[runvoid] Opened 2D Hardware Window [640x480] @ 60 FPS
[runvoid] GUI Event loop started successfully.`
  },

  pro: {
    code: `<span class="cmt">// Pro Systems Mode: Zero-GC & Raw Pointers</span>
<span class="dir">remove garbageC</span>
<span class="dir">remove Basic</span>
<span class="dir">add Advanced</span>
<span class="kw">use</span> ior
<span class="kw">use</span> mem

<span class="cmt">// Stack address and pointer mutation</span>
<span class="kw">remember</span> val: <span class="type">Int</span> = <span class="num">500</span>
<span class="kw">remember</span> ptr: <span class="type">Ptr</span> = <span class="kw">addr</span> val

@ptr = <span class="num">750</span>
<span class="kw">say</span> val  <span class="cmt">// prints 750</span>

<span class="cmt">// Direct heap allocation</span>
<span class="kw">remember</span> heap: <span class="type">Ptr</span> = <span class="kw">alloc</span> <span class="num">64</span>
@heap = <span class="num">12345</span>
<span class="kw">say</span> @heap
<span class="kw">free</span> heap

<span class="cmt">// Hardware assembly insertion</span>
<span class="kw">asm</span> {
    mov rax, 42
    imul rax, 10
}`,
    output: `[runvoid] Pro Mode Enabled: Zero-GC, Strict Typing, LTO (-O3)
[runvoid] Assembling user.asm with NASM...
[runvoid] Stripped ELF executable: 15.2 KB
[runvoid] Executing:

750
12345`
  },

  threads: {
    code: `<span class="cmt">// Native POSIX Multithreading & Hardware Atomics</span>
<span class="dir">remove garbageC</span>
<span class="dir">remove Basic</span>
<span class="dir">add Advanced</span>
<span class="kw">use</span> ior
<span class="kw">use</span> thread

<span class="kw">remember</span> shared_counter: <span class="type">Int</span> = <span class="num">0</span>

<span class="cmt">// Spawn native Linux thread</span>
<span class="kw">thread</span> {
    <span class="cmt">// Hardware instruction: lock add [r12 + offset], rax</span>
    <span class="kw">atomic add</span> shared_counter, <span class="num">100</span>
}

<span class="kw">wait</span> <span class="num">1</span>
<span class="kw">say</span> <span class="str">"Synchronized Atomic Counter: {shared_counter}"</span>`,
    output: `[runvoid] Compiling threads.rv with -lpthread...
[runvoid] Spawned pthread worker [tid: 18492]
[runvoid] Emitted hardware instruction: lock add [r12 - 8], rax
[runvoid] Executing:

Synchronized Atomic Counter: 100`
  },

  baremetal: {
    code: `<span class="cmt">// Bare-Metal Freestanding Binary (No libc, No runtime)</span>
<span class="dir">remove garbageC</span>
<span class="dir">remove Basic</span>
<span class="dir">remove Linux</span>
<span class="dir">add Advanced</span>
<span class="dir">add Freestanding</span>

<span class="cmt">// Entry point 'global _start' emitted automatically</span>
<span class="cmt">// Direct Linux exit syscall: rax=60, rdi=0</span>
<span class="kw">asm</span> {
    mov rax, 60
    xor rdi, rdi
    syscall
}`,
    output: `[runvoid] Freestanding Bare-Metal Mode:
[runvoid]   - Entry Point: _start
[runvoid]   - Libc: REMOVED (0 dependencies)
[runvoid]   - Linking directly with: ld -s
[runvoid] Standalone Binary Size: 8.4 KB
[runvoid] Process exited cleanly with status code 0`
  }
};

const installCommands = {
  arch: "sudo pacman -S rust cargo nasm gcc libx11",
  ubuntu: "sudo apt update && sudo apt install -y rustc cargo nasm gcc libx11-dev",
  fedora: "sudo dnf install -y rust cargo nasm gcc libX11-devel",
  cargo: "cargo install --git https://github.com/runvoid/runvoid.git"
};

const i18n = {
  en: {
    badge: "v0.2.0 • Pure x86_64 Native Compiler",
    heroTitle: "The Conversational Language With Machine Code Speed",
    heroSub: "Simple and intuitive like Python & Bash. Blazing fast, compact, and bare-metal like C & Assembly. Compiles directly to x86_64 NASM with zero virtual machine overhead.",
    btnGetStarted: "Get Started",
    btnDocs: "Documentation",
    statSize: "15 KB",
    statSizeLabel: "Standalone Executables",
    statVm: "0 ms",
    statVmLabel: "VM / Interpreter Overhead",
    statCode: "100%",
    statCodeLabel: "Pure x86_64 Machine Code",
    statTools: "50+",
    statToolsLabel: "Native Built-in Features",
    tagShowcase: "Interactive Code Showcase",
    titleShowcase: "From Plain English to Bare-Metal Machine Code",
    descShowcase: "Explore how Runvoid effortlessly adapts from a friendly beginner language to a high-performance bare-metal systems compiler.",
    tabBeginner: "👋 Beginner: Lists & Logic",
    tabCanvas: "🕹️ 2D Canvas & GUI",
    tabPro: "⚡ Pro: Pointers & Asm",
    tabThreads: "🧵 Multithreading & Atomics",
    tabBaremetal: "🛡️ Bare-Metal Freestanding",
    btnRun: "▶ Run",
    tagFeatures: "Why Runvoid?",
    titleFeatures: "Engineered for Beginners. Built for Systems.",
    feat1Title: "Conversational English Syntax",
    feat1Desc: "Write code that reads naturally: say, remember, repeat 5 times, for every item in backpack, if file exists.",
    feat2Title: "Pure x86_64 NASM Compiler",
    feat2Desc: "Translates code directly into handwritten-quality assembly. Zero bytecode interpreters or bulky garbage runtimes.",
    feat3Title: "Ultra-Lightweight 15 KB Binaries",
    feat3Desc: "Compiles into standalone static Linux ELF executables that run instantly on any machine with zero installation.",
    feat4Title: "Hardware 2D Canvas & Desktop GUI",
    feat4Desc: "Build arcade games and native windowed tools with hardware acceleration out of the box.",
    feat5Title: "POSIX Multithreading & Atomics",
    feat5Desc: "Spawn OS threads with shared memory and hardware-locked CPU atomic operations (lock add).",
    feat6Title: "Full Developer Ecosystem",
    feat6Desc: "Includes built-in code formatter (runvoid fmt), VS Code extension, template generator, and interactive cheat sheet.",
    tagBenchmark: "Performance & Size",
    titleBenchmark: "How Runvoid Compares",
    descBenchmark: "Runvoid combines scripting agility with the binary footprint and execution speed of compiled systems languages.",
    tagInstall: "Quick Setup",
    titleInstall: "Install in Seconds",
    descInstall: "Runvoid runs on any Linux x86_64 distribution with GCC and NASM.",
    footerNote: "Released under GNU General Public License v3.0 (GPL-3.0). Designed & built with Rust and NASM."
  },
  ru: {
    badge: "v0.2.0 • Чистый компилятор x86_64",
    heroTitle: "Разговорный язык со скоростью чистого ассемблера",
    heroSub: "Простой и понятный, как Python и Bash. Молниеносный, компактный и низкоуровневый, как C и Assembly. Компилируется сразу в машинный код x86_64 без виртуальных машин.",
    btnGetStarted: "Быстрый старт",
    btnDocs: "Документация",
    statSize: "15 КБ",
    statSizeLabel: "Размер автономных бинарников",
    statVm: "0 мс",
    statVmLabel: "Оверхед виртуальной машины",
    statCode: "100%",
    statCodeLabel: "Чистый машинный код x86_64",
    statTools: "50+",
    statToolsLabel: "Встроенных возможностей",
    tagShowcase: "Интерактивная демонстрация",
    titleShowcase: "От живого английского языка до голого железа",
    descShowcase: "Попробуйте, как Runvoid легко масштабируется от уютного языка для начинающих до бескомпромиссного системного компилятора.",
    tabBeginner: "👋 Для новичков: списки и логика",
    tabCanvas: "🕹️ 2D Графика и GUI",
    tabPro: "⚡ Pro: указатели и ассемблер",
    tabThreads: "🧵 Многопоточность и атомики",
    tabBaremetal: "🛡️ Bare-Metal без ОС",
    btnRun: "▶ Запустить",
    tagFeatures: "Преимущества Runvoid",
    titleFeatures: "Создан для новичков. Заточен под системы.",
    feat1Title: "Человечный разговорный синтаксис",
    feat1Desc: "Код пишется понятными фразами: say, remember, repeat 5 times, for every item in backpack, if file exists.",
    feat2Title: "Компиляция в чистый NASM x86_64",
    feat2Desc: "Транслирует код сразу в качественный ассемблер. Никаких интерпретаторов байткода или тяжелых сред исполнения.",
    feat3Title: "Микроскопические бинарники от 15 КБ",
    feat3Desc: "Собирает независимые ELF-файлы Linux, которые мгновенно запускаются на любом сервере без установки зависимостей.",
    feat4Title: "Аппаратный 2D холст и оконный GUI",
    feat4Desc: "Создавайте аркадные 2D игры и оконные десктопные панели с кнопками и флажками прямо из коробки.",
    feat5Title: "Нативные потоки ОС и атомики",
    feat5Desc: "Запуск параллельных потоков с общей памятью и аппаратными атомарными инструкциями процессора (lock add).",
    feat6Title: "Готовая экосистема разработки",
    feat6Desc: "Включает автоформатер (runvoid fmt), расширение для VS Code, генератор шаблонов и интерактивную шпаргалку.",
    tagBenchmark: "Сравнение и скорость",
    titleBenchmark: "Как Runvoid выглядит в сравнении",
    descBenchmark: "Runvoid сочетает простоту скриптовых языков со скоростью и минимальным размером компилируемых систем.",
    tagInstall: "Быстрая установка",
    titleInstall: "Установка за пару секунд",
    descInstall: "Runvoid работает на любых дистрибутивах Linux x86_64 с установленными GCC и NASM.",
    footerNote: "Лицензия GNU General Public License v3.0 (GPL-3.0). Написан на Rust и NASM."
  }
};

let currentLang = "en";
let currentTab = "beginner";

function switchTab(tabKey) {
  currentTab = tabKey;
  document.querySelectorAll(".tab-btn").forEach(btn => {
    btn.classList.toggle("active", btn.dataset.tab === tabKey);
  });
  
  const codeEl = document.getElementById("codeDisplay");
  const outputEl = document.getElementById("terminalDisplay");
  
  codeEl.innerHTML = snippets[tabKey].code;
  outputEl.textContent = snippets[tabKey].output;
}

function runCodeSimulation() {
  const outputEl = document.getElementById("terminalDisplay");
  outputEl.textContent = "[runvoid] Compiling with Rust & NASM...\n[runvoid] Optimizing AST (constant folding & DCE)...";
  
  setTimeout(() => {
    outputEl.textContent = snippets[currentTab].output;
  }, 350);
}

function switchInstallTab(distro) {
  document.querySelectorAll(".install-tab").forEach(btn => {
    btn.classList.toggle("active", btn.dataset.distro === distro);
  });
  document.getElementById("installCmd").textContent = installCommands[distro];
}

function copyInstallCmd() {
  const cmd = document.getElementById("installCmd").textContent;
  navigator.clipboard.writeText(cmd).then(() => {
    const copyBtn = document.getElementById("copyBtn");
    const orig = copyBtn.textContent;
    copyBtn.textContent = currentLang === "ru" ? "Скопировано!" : "Copied!";
    setTimeout(() => {
      copyBtn.textContent = orig;
    }, 2000);
  });
}

function toggleLanguage() {
  currentLang = currentLang === "en" ? "ru" : "en";
  document.getElementById("langBtn").textContent = currentLang === "en" ? "🌐 RU" : "🌐 EN";
  
  const texts = i18n[currentLang];
  document.querySelectorAll("[data-i18n]").forEach(el => {
    const key = el.dataset.i18n;
    if (texts[key]) {
      el.textContent = texts[key];
    }
  });

  // Update doc links
  const docLinkNav = document.getElementById("docLinkNav");
  const docLinkHero = document.getElementById("docLinkHero");
  const docLinkFooter = document.getElementById("docLinkFooter");
  
  const targetDoc = currentLang === "ru" ? "DOCUMENTATION_RU.md" : "DOCUMENTATION.md";
  if (docLinkNav) docLinkNav.href = targetDoc;
  if (docLinkHero) docLinkHero.href = targetDoc;
  if (docLinkFooter) docLinkFooter.href = targetDoc;
}

document.addEventListener("DOMContentLoaded", () => {
  switchTab("beginner");
  switchInstallTab("arch");
  
  document.querySelectorAll(".tab-btn").forEach(btn => {
    btn.addEventListener("click", () => switchTab(btn.dataset.tab));
  });
  
  document.querySelectorAll(".install-tab").forEach(btn => {
    btn.addEventListener("click", () => switchInstallTab(btn.dataset.distro));
  });
  
  document.getElementById("runBtn").addEventListener("click", runCodeSimulation);
  document.getElementById("copyBtn").addEventListener("click", copyInstallCmd);
  document.getElementById("langBtn").addEventListener("click", toggleLanguage);
});
