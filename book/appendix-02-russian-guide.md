# Приложение Б: Полное руководство на русском языке

Добро пожаловать в официальную документацию языка **Runvoid**! 
Runvoid создан так, чтобы программирование было понятным, невероятно быстрым и приносящим удовольствие.

---

## 1. Быстрый старт за 5 минут

### Первая программа
Создайте файл `hello.rv`:
```runvoid
say "Привет, прекрасный мир Runvoid!"
```
Запустите прямо из консоли:
```bash
runvoid run hello.rv
```

### Переменные («Запомни»)
В Runvoid переменные объявляются понятным словом `remember`:
```runvoid
remember player = "Алекс"
remember score = 100

say "Игрок: {player}, Очки: {score}"

score = score + 50
say "Новый счет: {score}"
```

### Ввод пользователя («Спроси»)
```runvoid
remember name = ask "Как зовут твоего героя? "
say "Добро пожаловать в игру, {name}!"
```

### Условия и ветвления
```runvoid
remember gold = 75

if gold >= 100 {
    say "Ты можешь купить легендарный меч!"
} otherwise if gold >= 50 {
    say "Золота хватает на прочный щит."
} otherwise {
    say "Копи монеты, искатель приключений."
}
```

---

## 2. Разговорные списки

Списки работают так же просто, как список покупок в блокноте:

```runvoid
remember inventory = "Факел", "Щит", "Зелье здоровья"

add "Магическое кольцо" to inventory
remove "Щит" from inventory

if inventory has "Факел" {
    say "В подземелье будет светло!"
}

say "Всего предметов: {count inventory}"

for every item in inventory {
    say " - {item}"
}
```

---

## 3. Системный Pro-режим (`add Advanced`)

Для максимальной производительности, прямого доступа к регистрам процессора и ручного управления памятью:

```runvoid
remove garbageC   # Отключает сборщик мусора
remove Basic      # Включает строгую статическую типизацию
add Advanced      # Включает системные возможности
use ior
use mem

# Сырые указатели:
remember val: Int = 500
remember ptr: Ptr = addr val

say @ptr       // 500
@ptr = 750     // запись по адресу
say val        // 750

# Ручное выделение в куче:
remember heap: Ptr = alloc 64
@heap = 9999
free heap

# Встроенный ассемблер:
asm {
    mov rax, 42
    imul rax, 10
}
```

---

## 4. Bare-Metal режим без ОС

Сборка бинарника без libc и без сторонних рантаймов:

```runvoid
remove garbageC
remove Basic
remove Linux
add Advanced
add Freestanding

asm {
    mov rax, 60    ; Linux exit syscall
    xor rdi, rdi   ; code 0
    syscall
}
```
Компилятор генерирует точку входа `_start` и компонует автономный ELF напрямую через `ld -s`.
