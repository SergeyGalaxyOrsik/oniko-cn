# Компоненты oniko-cn

## Готово

| Компонент | Исходник в реестре | Установка | Платформы |
|---|---|---|---|
| Button | `registry/ui/button.rs` | `oniko add button` | web, desktop |

### Button

- Варианты (`ButtonVariant`): `Default`, `Outline`, `Secondary`, `Ghost`, `Destructive`, `Link`
- Размеры (`ButtonSize`): `Default`, `Xs`, `Sm`, `Lg`, `Icon`, `IconXs`, `IconSm`, `IconLg`
- Хелпер `button_variants(variant, size, class)`: классы кнопки для любого элемента, например для ссылки
- Доступен во всех 8 стилях: vega, nova, maia, lyra, mira, luma, rhea, sera
