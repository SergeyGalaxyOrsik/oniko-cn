# Компоненты oniko-cn

## Готово

| Компонент | Исходник в реестре | Установка | Платформы |
|---|---|---|---|
| Button | `registry/ui/button.rs` | `oniko add button` | web, desktop |
| Alert | `registry/ui/alert.rs` | `oniko add alert` | web, desktop |
| Alert Dialog | `registry/ui/alert_dialog.rs` | `oniko add alert-dialog` | web, desktop |
| Aspect Ratio | `registry/ui/aspect_ratio.rs` | `oniko add aspect-ratio` | web, desktop |
| Avatar | `registry/ui/avatar.rs` | `oniko add avatar` | web, desktop |
| Accordion | `registry/ui/accordion.rs` | `oniko add accordion` | web, desktop |
| Badge | `registry/ui/badge.rs` | `oniko add badge` | web, desktop |
| Bubble | `registry/ui/bubble.rs` | `oniko add bubble` | web, desktop |
| Breadcrumb | `registry/ui/breadcrumb.rs` | `oniko add breadcrumb` | web, desktop |
| Button Group | `registry/ui/button_group.rs` | `oniko add button-group` | web, desktop |
| Input Group | `registry/ui/input_group.rs` | `oniko add input-group` | web, desktop |
| Checkbox | `registry/ui/checkbox.rs` | `oniko add checkbox` | web, desktop |
| Combobox | `registry/ui/combobox.rs` | `oniko add combobox` | web, desktop |
| Code Block | `registry/ui/code_block.rs` | `oniko add code-block` | web, desktop |
| Collapsible | `registry/ui/collapsible.rs` | `oniko add collapsible` | web, desktop |
| Command | `registry/ui/command.rs` | `oniko add command` | web, desktop |
| Table | `registry/ui/table.rs` | `oniko add table` | web, desktop |
| Data Table | `registry/ui/data_table.rs` | `oniko add data-table` | web, desktop |
| Drawer | `registry/ui/drawer.rs` | `oniko add drawer` | web, desktop |
| Dialog | `registry/ui/dialog.rs` | `oniko add dialog` | web, desktop |
| Dropdown Menu | `registry/ui/dropdown_menu.rs` | `oniko add dropdown-menu` | web, desktop |
| Card | `registry/ui/card.rs` | `oniko add card` | web, desktop |
| Calendar | `registry/ui/calendar.rs` | `oniko add calendar` | web, desktop |
| Carousel | `registry/ui/carousel.rs` | `oniko add carousel` | web, desktop |
| Popover | `registry/ui/popover.rs` | `oniko add popover` | web, desktop |
| Item | `registry/ui/item.rs` | `oniko add item` | web, desktop |
| Separator | `registry/ui/separator.rs` | `oniko add separator` | web, desktop |
| Date Picker | `registry/ui/date_picker.rs` | `oniko add date-picker` | web, desktop |
| Context Menu | `registry/ui/context_menu.rs` | `oniko add context-menu` | web, desktop |
| Empty | `registry/ui/empty.rs` | `oniko add empty` | web, desktop |
| Hover Card | `registry/ui/hover_card.rs` | `oniko add hover-card` | web, desktop |
| Field | `registry/ui/field.rs` | `oniko add field` | web, desktop |
| Input OTP | `registry/ui/input_otp.rs` | `oniko add input-otp` | web, desktop |
| Input | `registry/ui/input.rs` | `oniko add input` | web, desktop |

### Button

- Варианты (`ButtonVariant`): `Default`, `Outline`, `Secondary`, `Ghost`, `Destructive`, `Link`
- Размеры (`ButtonSize`): `Default`, `Xs`, `Sm`, `Lg`, `Icon`, `IconXs`, `IconSm`, `IconLg`
- Хелпер `button_variants(variant, size, class)`: классы кнопки для любого элемента, например для ссылки
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Alert

- Варианты (`AlertVariant`): `Default`, `Destructive`
- Части: `Alert`, `AlertTitle`, `AlertDescription`, `AlertAction`
- Иконка передаётся первым дочерним `svg` внутри `Alert`
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Aspect Ratio

- Проп `ratio: f64` — ширина / высота, например `16.0 / 9.0`; значение попадает в CSS-переменную `--ratio`
- Стилей нет: компонент одинаков во всех 8 стилях

### Accordion

- Части: `Accordion`, `AccordionItem`, `AccordionTrigger`, `AccordionContent`
- `Accordion`: `multiple` (несколько открытых пунктов), `disabled`, `default_value: Vec<String>`, `on_value_change`
- `AccordionItem`: обязательный `value`, `disabled`
- Анимация высоты: `AccordionTrigger` перед переключением измеряет контент через `MountedData::get_scroll_size` и кладёт результат в `--accordion-panel-height`
- Закрытый контент получает `inert`, поэтому не попадает в порядок табуляции
- Без `dioxus-primitives` (на crates.io лежит пустышка 0.0.0), стрелки вверх/вниз между заголовками не реализованы: работают Tab, Enter, Space
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Alert Dialog

- Части: `AlertDialog`, `AlertDialogTrigger`, `AlertDialogContent`, `AlertDialogHeader`, `AlertDialogFooter`, `AlertDialogMedia`, `AlertDialogTitle`, `AlertDialogDescription`, `AlertDialogAction`, `AlertDialogCancel`
- Размеры `AlertDialogContent` (`AlertDialogSize`): `Default`, `Sm`
- Состояние: `default_open`, либо управляемое через `open` и `on_open_change`
- Построен на нативном `<dialog>` с `showModal()`: фокус-ловушку и `Escape` даёт браузер или webview, а не `dioxus-primitives`, потому что реестр не умеет git-зависимости
- `AlertDialogAction` и `AlertDialogCancel` вызывают `onclick` и закрывают диалог; закрытие кликом по фону не поддерживается
- Зависит от `button` (ставится автоматически)
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Badge

- Варианты (`BadgeVariant`): `Default`, `Secondary`, `Destructive`, `Outline`, `Ghost`, `Link`
- Хелпер `badge_variants(variant, class)`: классы бейджа для любого элемента, например для ссылки
- Иконка передаётся дочерним `svg`; отступ под неё задают `data-icon="inline-start"` / `data-icon="inline-end"` на самом `svg`
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Avatar

- Размеры (`AvatarSize`): `Default`, `Sm`, `Lg` (попадает в `data-size`)
- Части: `Avatar`, `AvatarImage`, `AvatarFallback`, `AvatarBadge`, `AvatarGroup`, `AvatarGroupCount`
- `AvatarFallback` виден, пока `AvatarImage` не загрузился, и остаётся при ошибке загрузки
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Breadcrumb

- Части: `Breadcrumb`, `BreadcrumbList`, `BreadcrumbItem`, `BreadcrumbLink`, `BreadcrumbPage`, `BreadcrumbSeparator`, `BreadcrumbEllipsis`
- `BreadcrumbSeparator`: по умолчанию рисует шеврон (в RTL он разворачивается), либо любые дочерние элементы
- `BreadcrumbLink` — обычный `<a>`; у shadcn есть проп `render`, здесь его нет: чтобы подставить роутер-ссылку, отредактируйте файл
- `BreadcrumbPage` помечает текущую страницу: `aria-current="page"`
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Collapsible

- Части: `Collapsible`, `CollapsibleTrigger`, `CollapsibleContent`
- `Collapsible`: `default_open` (без управления извне), `open: Option<bool>` с `on_open_change` (управляемое состояние), `disabled`
- Закрытый `CollapsibleContent` не рендерится, поэтому не попадает в порядок табуляции
- Стилей нет: компонент одинаков во всех 8 стилях
- Без `dioxus-primitives` (на crates.io лежит пустышка 0.0.0), состояние и `aria-expanded` ведутся самим компонентом

### Command

- Части: `Command`, `CommandInput`, `CommandList`, `CommandEmpty`, `CommandGroup`, `CommandItem`, `CommandSeparator`, `CommandShortcut`
- `CommandGroup`: `heading: Option<String>`; группа скрывается, если ни один её пункт не подходит под запрос
- `CommandItem`: обязательный `value` (ищется по нему и по `keywords`), `keywords`, `disabled`, `on_select: EventHandler<String>`
- Поиск: подстрока без учёта регистра, без нечёткого ранжирования как в cmdk; порядок результатов — порядок на экране
- Стрелки вверх/вниз двигают подсветку по видимым пунктам, `Enter` выбирает подсвеченный (или первый подходящий), наведение мыши тоже подсвечивает
- `CommandDialog` (командная палитра в диалоге) не реализован: собирается из `Dialog` и `Command` вручную
- Без `dioxus-primitives` (на crates.io лежит пустышка 0.0.0): состояние и клавиатура ведутся самим компонентом
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Button Group

- Ориентация (`ButtonGroupOrientation`): `Horizontal` (по умолчанию), `Vertical`; попадает в `data-orientation`
- Части: `ButtonGroup`, `ButtonGroupText`, `ButtonGroupSeparator`
- `ButtonGroupSeparator`: по умолчанию вертикальный (`Vertical`), как у shadcn
- Кнопки внутри группы склеиваются: у средних убираются скругления и общие границы
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Bubble

- Варианты (`BubbleVariant`): `Default`, `Secondary`, `Muted`, `Tinted`, `Outline`, `Ghost`, `Destructive`
- Выравнивание `Bubble` (`BubbleAlign`): `Start`, `End`; попадает в `data-align`
- Части: `BubbleGroup`, `Bubble`, `BubbleContent`, `BubbleContentButton`, `BubbleContentLink`, `BubbleReactions`
- Вместо `render` из React тело пузыря бывает тремя компонентами: `div`, `button` и `a`; хелпер `bubble_content_variants(class)` даёт классы для любого другого элемента
- `BubbleReactions`: `side` (`BubbleReactionsSide`: `Top`, `Bottom`) и `align` (`BubbleAlign`, по умолчанию `End`); позиционируется абсолютно относительно `Bubble`
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Checkbox

- Проп `checked: Option<bool>` делает компонент управляемым; без него состояние хранится внутри, начальное значение задаёт `default_checked`
- `on_checked_change: EventHandler<bool>` получает новое значение; `disabled` блокирует переключение
- Построен на `<button role="checkbox">` с `aria-checked`: Space и Enter переключают через нативное поведение кнопки; `dioxus-primitives` не используется (на crates.io пустышка 0.0.0)
- Скрытого `<input>` для форм нет: значение не уходит в `FormData`, его нужно передавать вручную через `on_checked_change`

### Code Block

- Варианты (`CodeBlockVariant`): `Default`, `Destructive`; размеры (`CodeBlockSize`): `Default`, `Sm`
- Части: `CodeBlock`, `CodeBlockHeader`, `CodeBlockTitle`, `CodeBlockActions`, `CodeBlockCopyButton`, `CodeBlockContent`, `CodeBlockLine`, `CodeBlockFooter`
- `CodeBlockCopyButton` копирует `value` через `navigator.clipboard` (`document::eval`); `CodeBlockActions` внутри `CodeBlock` всплывает поверх содержимого
- `CodeBlockContent`: `wrap`, `line_numbers`, `start_line`; `CodeBlockLine` варианты (`CodeBlockLineVariant`): `Default`, `Added`, `Removed`, `Highlighted`
- Подсветка синтаксиса не входит в компонент: текст выводится как есть
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Combobox

- Части: `Combobox`, `ComboboxInput`, `ComboboxTrigger`, `ComboboxContent`, `ComboboxList`, `ComboboxItem`, `ComboboxGroup`, `ComboboxLabel`, `ComboboxEmpty`, `ComboboxSeparator`
- `Combobox`: `value: Option<String>` делает выбор управляемым (`None` значит «без управления»), `default_value` задаёт начальный выбор, `on_value_change: EventHandler<String>`, `disabled`
- `ComboboxItem`: `value` — что сохраняется; `label` — текст для поиска и для поля после выбора (по умолчанию равен `value`). Передавайте `label`, если содержимое пункта не совпадает с текстом
- Элементы регистрируются, пока список открыт; стрелки вверх и вниз, Enter, Escape и поиск по вводу работают только по отрисованным пунктам
- `ComboboxEmpty` кладётся внутрь `ComboboxContent`: показывается, когда ни один пункт не подходит
- Список — абсолютно позиционированный блок под полем (без JS-библиотек); множественный выбор (`Chips`) и кнопка очистки у shadcn здесь не реализованы
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Table

- Части: `Table`, `TableHeader`, `TableBody`, `TableFooter`, `TableRow`, `TableHead`, `TableCell`, `TableCaption`
- `Table` оборачивает `<table>` в контейнер с горизонтальной прокруткой
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Data Table

- Компонент `DataTable<T>`: `columns: Vec<DataColumn<T>>`, `data: Vec<T>`, `page_size` (по умолчанию 10), `selectable`, `filter_placeholder`
- `DataColumn::new(заголовок, |row| текст)`: текст используется для сортировки и фильтра; `.cell(|row| Element)` меняет только отрисовку ячейки; `.sortable()` включает сортировку по клику на заголовок (ascending, descending, без сортировки)
- Фильтр ищет подстроку во всех колонках без учёта регистра; числа сортируются как числа
- `selectable`: колонка с чекбоксами; выбор хранится по позиции строки в `data`, «выбрать всё» относится к строкам текущей страницы
- Пагинация: «Previous» и «Next»; номер страницы сбрасывается при изменении фильтра
- Нет скрытия колонок, отдельного select размера страницы и неопределённого состояния у чекбокса «выбрать всё» (в `Checkbox` его нет)
- Зависит от `button`, `checkbox`, `table` (ставятся автоматически)
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Card

- Размеры (`CardSize`): `Default`, `Sm` (попадает в `data-size`, стили уменьшают отступы)
- Части: `Card`, `CardHeader`, `CardTitle`, `CardDescription`, `CardAction`, `CardContent`, `CardFooter`
- `CardAction` встаёт в правый верхний угол `CardHeader`
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Calendar

- Один компонент `Calendar`: сетка месяца с выбором одного дня, неделя начинается с воскресенья
- Пропы: `selected: Option<CalendarDate>` (следует за пропом), `on_select: EventHandler<CalendarDate>`, `default_month` (какой месяц показать первым), `today` (помечает `data-today`), `min` / `max`, `disabled_dates: Vec<CalendarDate>`, `disabled`
- `CalendarDate` (`year`, `month`, `day`) делает даты без `chrono` и без часов: сегодняшний день задаёт приложение через `today`
- Навигация по месяцам через `button_variants` из `button.rs`; кнопки дней нативные, выбор по Enter и Space
- Стрелочной навигации с клавиатуры по сетке нет (`dioxus-primitives` в репозитории — пустышка 0.0.0)
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Carousel

- Ориентация (`CarouselOrientation`): `Horizontal` (по умолчанию), `Vertical`
- Части: `Carousel`, `CarouselContent`, `CarouselItem`, `CarouselPrevious`, `CarouselNext`
- Состояние текущего слайда в контексте `Carousel`, прокрутка через `transform`; стрелки клавиатуры работают внутри карусели
- Без зацикливания: кнопки отключаются на первом и последнем слайде
- Для вертикальной карусели задайте высоту на `CarouselContent` (например, `h-[200px]`)
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Dialog

- Части: `Dialog`, `DialogTrigger`, `DialogContent`, `DialogHeader`, `DialogFooter`, `DialogTitle`, `DialogDescription`, `DialogClose`
- Состояние: `default_open`, либо управляемое через `open` и `on_open_change`
- `DialogContent`: `show_close_button` (по умолчанию `true`) показывает крестик в углу
- Построен на нативном `<dialog>` с `showModal()`: фокус-ловушку и `Escape` даёт браузер или webview, а не `dioxus-primitives`, потому что реестр не умеет git-зависимости
- Закрытие кликом по затемнённому фону за окном, по `Escape`, по крестику и по `DialogClose`
- Зависит от `button` (ставится автоматически)
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Drawer

- Части: `Drawer`, `DrawerTrigger`, `DrawerContent`, `DrawerHeader`, `DrawerFooter`, `DrawerTitle`, `DrawerDescription`, `DrawerClose`
- `Drawer`: `direction` (`DrawerDirection`: `Down` по умолчанию, `Up`, `Left`, `Right`), `show_handle` (полоска-ручка сверху панели), `default_open`, либо управляемое через `open` и `on_open_change`
- Построен на нативном `<dialog>` с `showModal()`, как `AlertDialog`: фокус-ловушку и `Escape` даёт браузер или webview
- Закрытие по `Escape`, по затемнённому фону и по `DrawerClose`
- Жестов свайпа и точек фиксации (snap points) нет
- Зависит от `button` (ставится автоматически)
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Popover

- Части: `Popover`, `PopoverTrigger`, `PopoverContent`
- `Popover`: `default_open`, либо управляемое через `open` и `on_open_change`
- Панель закрывается по клику вне её (прозрачный слой на весь экран), по `Escape`
- Позиция: под триггером, CSS; переворота у края окна и `dioxus-primitives` нет
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Date Picker

- Один компонент `DatePicker`: кнопка-триггер, по клику открывает `Calendar` во `Popover`
- Пропы: `selected` (управляемое значение) или `default_selected`, `on_select`, `default_month`, `today`, `min`, `max`, `disabled_dates`, `placeholder` (по умолчанию `Pick a date`), `disabled`
- Атрибуты (например, `class`) уходят на кнопку-триггер
- Выбор дня закрывает панель
- Только одиночный выбор: диапазон, поле ввода и выбор времени не реализованы
- Зависит от `button`, `calendar`, `popover` (ставятся автоматически)
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Context Menu

- Части: `ContextMenu`, `ContextMenuTrigger`, `ContextMenuContent`, `ContextMenuItem`, `ContextMenuCheckboxItem`, `ContextMenuRadioGroup`, `ContextMenuRadioItem`, `ContextMenuLabel`, `ContextMenuSeparator`, `ContextMenuShortcut`, `ContextMenuGroup`
- `ContextMenu`: `open: Option<bool>` с `on_open_change` (управляемое состояние) или без них (внутреннее состояние)
- Открывается по правому клику (`oncontextmenu`) у `ContextMenuTrigger` в координатах курсора; повторный правый клик при открытом меню переносит его
- Закрывается по клику вне меню, `Escape`, `Tab` и после выбора пункта; стрелки `ArrowUp`/`ArrowDown` переходят по пунктам, наведение мыши тоже фокусирует пункт
- `ContextMenuItem`: `variant` (`ContextMenuItemVariant`: `Default`, `Destructive`), `inset`, `disabled`, `onclick`
- `ContextMenuCheckboxItem`: `checked` (управляемое) или `default_checked`, `on_checked_change`
- `ContextMenuRadioGroup`: `value` (управляемое) или `default_value`, `on_value_change`; `ContextMenuRadioItem` задаёт `value`
- Без подменю (`ContextMenuSub`): нужен отдельный механизм позиционирования, не реализован
- Позиция: `position: fixed` в точке курсора; переворота у края окна нет
- Без `dioxus-primitives` (на crates.io лежит пустышка 0.0.0)
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Hover Card

- Части: `HoverCard`, `HoverCardTrigger`, `HoverCardContent`
- `HoverCard`: `open_delay` (700 мс по умолчанию), `close_delay` (300 мс), `default_open`, либо управляемое через `open` и `on_open_change`
- `HoverCardContent`: `side` (`Top`, `Right`, `Bottom` по умолчанию, `Left`) и `align` (`Start`, `Center` по умолчанию, `End`)
- Открывается при наведении указателя и фокусе на триггер, закрывается по уходу указателя с триггера и карточки, по `Escape`
- Задержки работают через `setTimeout` в `document::eval`; переворота у края окна и `dioxus-primitives` нет
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Empty

- Части: `Empty`, `EmptyHeader`, `EmptyMedia`, `EmptyTitle`, `EmptyDescription`, `EmptyContent`
- Варианты медиа (`EmptyMediaVariant`): `Default`, `Icon` (попадает в `data-variant`, иконка в квадрате с фоном)
- `EmptyContent` для кнопок, полей ввода и ссылок
- Не зависит от других компонентов
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Field

- Части: `Field`, `FieldSet`, `FieldLegend` (`FieldLegendVariant`: `Legend`, `Label`), `FieldGroup`, `FieldContent`, `FieldLabel`, `FieldTitle`, `FieldDescription`, `FieldSeparator`, `FieldError`
- `Field`: `orientation` (`FieldOrientation`: `Vertical`, `Horizontal`, `Responsive`), `invalid`, `disabled`; состояние попадает в `data-invalid` и `data-disabled`
- `FieldLabel` принимает `r#for` для связи с элементом управления
- `FieldSeparator`: необязательный текст по центру линии
- `FieldError`: выводит сообщение как `role="alert"`; список ошибок не поддерживается, передавайте готовый текст
- Не зависит от других компонентов: подпись и разделитель написаны на обычных элементах
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Input

- Однострочное текстовое поле; без вариантов и размеров
- Атрибуты HTML (`r#type`, `placeholder`, `disabled`, `aria-invalid`, `value`) передаются напрямую компоненту
- Обработчики: `oninput`, `onchange`, `onfocus`, `onblur`, `onkeydown`, `onclick` (`Option<EventHandler<...>>`)
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Item

- Варианты (`ItemVariant`): `Default`, `Outline`, `Muted`; размеры (`ItemSize`): `Default`, `Sm`, `Xs`; попадают в `data-variant` и `data-size`
- Части: `Item`, `ItemGroup`, `ItemSeparator`, `ItemMedia` (`ItemMediaVariant`: `Default`, `Icon`, `Image`), `ItemContent`, `ItemTitle`, `ItemDescription`, `ItemActions`, `ItemHeader`, `ItemFooter`
- Вместо `render` из React хелпер `item_variants(variant, size, class)` даёт классы для ссылки или другого элемента, который должен выглядеть как `Item`
- `ItemSeparator` делит элементы внутри `ItemGroup`; зависит от `separator` (ставится автоматически)
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Separator

- Ориентация (`SeparatorOrientation`): `Horizontal` (по умолчанию), `Vertical`; попадает в `data-orientation`
- `separator_variants(orientation, class)` даёт классы для собственных элементов-разделителей
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera

### Input OTP

- Части: `InputOTP`, `InputOTPGroup`, `InputOTPSlot`, `InputOTPSeparator`
- `InputOTP`: `max_length` (по умолчанию 6), `pattern` (`OtpPattern`: `Any`, `Digits`, `DigitsAndChars`), `value` (управляемое значение), `default_value`, `on_value_change`, `on_complete` (вызывается один раз при заполнении), `disabled`, `name` (попадает в `FormData`), `aria_label`
- `InputOTPSlot`: `index` (позиция в поле), `invalid` (ставит `aria-invalid`, слот красится в цвет ошибки)
- Ввод идёт в один прозрачный `input` поверх слотов: работают клавиатура, вставка и автозаполнение кода (`autocomplete="one-time-code"`), без `web_sys`
- Курсор показывается в первом пустом слоте, а при полном поле — в последнем
- Редактирование в середине значения не поддерживается: Backspace удаляет последний символ
- Не зависит от других компонентов
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera
