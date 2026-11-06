//! Typography examples, ported from shadcn/ui `examples/base/typography-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::typography::{
    TypographyBlockquote, TypographyH1, TypographyH2, TypographyH3, TypographyH4,
    TypographyInlineCode, TypographyLarge, TypographyLead, TypographyList, TypographyMuted,
    TypographyP, TypographySmall, TypographyTable, TypographyTableCell, TypographyTableHead,
    TypographyTableRow,
};

#[component]
pub fn Examples() -> Element {
    rsx! {
        TypographyDemo {}
        TypographyH1Example {}
        TypographyH2Example {}
        TypographyH3Example {}
        TypographyH4Example {}
        TypographyPExample {}
        TypographyBlockquoteExample {}
        TypographyTableExample {}
        TypographyListExample {}
        TypographyInlineCodeExample {}
        TypographyLeadExample {}
        TypographyLargeExample {}
        TypographySmallExample {}
        TypographyMutedExample {}
        TypographyRtl {}
    }
}

const QUOTE: &str = "\u{201C}After all,\u{201D} he said, \u{201C}everyone enjoys a good joke, so it\u{2019}s only fair that they should pay for the privilege.\u{201D}";

#[component]
fn KingsTable() -> Element {
    rsx! {
        TypographyTable {
            thead {
                TypographyTableRow {
                    TypographyTableHead { "King\u{2019}s Treasury" }
                    TypographyTableHead { "People\u{2019}s happiness" }
                }
            }
            tbody {
                TypographyTableRow {
                    TypographyTableCell { "Empty" }
                    TypographyTableCell { "Overflowing" }
                }
                TypographyTableRow {
                    TypographyTableCell { "Modest" }
                    TypographyTableCell { "Satisfied" }
                }
                TypographyTableRow {
                    TypographyTableCell { "Full" }
                    TypographyTableCell { "Ecstatic" }
                }
            }
        }
    }
}

#[component]
fn TypographyDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "h-auto",
            div {
                TypographyH1 { "Taxing Laughter: The Joke Tax Chronicles" }
                TypographyLead { class: "leading-7 [&:not(:first-child)]:mt-6",
                    "Once upon a time, in a far-off land, there was a very lazy king who spent all day lounging on his throne. One day, his advisors came to him with a problem: the kingdom was running out of money."
                }
                TypographyH2 { class: "mt-10 transition-colors", "The King\u{2019}s Plan" }
                TypographyP {
                    "The king thought long and hard, and finally came up with "
                    a {
                        href: "#",
                        class: "font-medium text-primary underline underline-offset-4",
                        "a brilliant plan"
                    }
                    ": he would tax the jokes in the kingdom."
                }
                TypographyBlockquote { "{QUOTE}" }
                TypographyH3 { class: "mt-8", "The Joke Tax" }
                TypographyP {
                    "The king\u{2019}s subjects were not amused. They grumbled and complained, but the king was firm:"
                }
                TypographyList {
                    li { "1st level of puns: 5 gold coins" }
                    li { "2nd level of jokes: 10 gold coins" }
                    li { "3rd level of one-liners : 20 gold coins" }
                }
                TypographyP {
                    "As a result, people stopped telling jokes, and the kingdom fell into a gloom. But there was one person who refused to let the king\u{2019}s foolishness get him down: a court jester named Jokester."
                }
                TypographyH3 { class: "mt-8", "Jokester\u{2019}s Revolt" }
                TypographyP {
                    "Jokester began sneaking into the castle in the middle of the night and leaving jokes all over the place: under the king\u{2019}s pillow, in his soup, even in the royal toilet. The king was furious, but he couldn\u{2019}t seem to stop Jokester."
                }
                TypographyP {
                    "And then, one day, the people of the kingdom discovered that the jokes left by Jokester were so funny that they couldn\u{2019}t help but laugh. And once they started laughing, they couldn\u{2019}t stop."
                }
                TypographyH3 { class: "mt-8", "The People\u{2019}s Rebellion" }
                TypographyP {
                    "The people of the kingdom, feeling uplifted by the laughter, started to tell jokes and puns again, and soon the entire kingdom was in on the joke."
                }
                KingsTable {}
                TypographyP {
                    "The king, seeing how much happier his subjects were, realized the error of his ways and repealed the joke tax. Jokester was declared a hero, and the kingdom lived happily ever after."
                }
                TypographyP {
                    "The moral of the story is: never underestimate the power of a good laugh and always be careful of bad ideas."
                }
            }
        }
    }
}

#[component]
fn TypographyH1Example() -> Element {
    rsx! {
        Example { title: "h1",
            TypographyH1 { class: "text-center", "Taxing Laughter: The Joke Tax Chronicles" }
        }
    }
}

#[component]
fn TypographyH2Example() -> Element {
    rsx! {
        Example { title: "h2",
            TypographyH2 { "The People of the Kingdom" }
        }
    }
}

#[component]
fn TypographyH3Example() -> Element {
    rsx! {
        Example { title: "h3",
            TypographyH3 { "The Joke Tax" }
        }
    }
}

#[component]
fn TypographyH4Example() -> Element {
    rsx! {
        Example { title: "h4",
            TypographyH4 { "People stopped telling jokes" }
        }
    }
}

#[component]
fn TypographyPExample() -> Element {
    rsx! {
        Example { title: "p",
            TypographyP {
                "The king, seeing how much happier his subjects were, realized the error of his ways and repealed the joke tax."
            }
        }
    }
}

#[component]
fn TypographyBlockquoteExample() -> Element {
    rsx! {
        Example { title: "blockquote",
            TypographyBlockquote { "{QUOTE}" }
        }
    }
}

#[component]
fn TypographyTableExample() -> Element {
    rsx! {
        Example { title: "table",
            KingsTable {}
        }
    }
}

#[component]
fn TypographyListExample() -> Element {
    rsx! {
        Example { title: "list",
            TypographyList {
                li { "1st level of puns: 5 gold coins" }
                li { "2nd level of jokes: 10 gold coins" }
                li { "3rd level of one-liners : 20 gold coins" }
            }
        }
    }
}

#[component]
fn TypographyInlineCodeExample() -> Element {
    rsx! {
        Example { title: "Inline code",
            TypographyInlineCode { "@radix-ui/react-alert-dialog" }
        }
    }
}

#[component]
fn TypographyLeadExample() -> Element {
    rsx! {
        Example { title: "Lead",
            TypographyLead {
                "A modal dialog that interrupts the user with important content and expects a response."
            }
        }
    }
}

#[component]
fn TypographyLargeExample() -> Element {
    rsx! {
        Example { title: "Large",
            TypographyLarge { "Are you absolutely sure?" }
        }
    }
}

#[component]
fn TypographySmallExample() -> Element {
    rsx! {
        Example { title: "Small",
            TypographySmall { "Email address" }
        }
    }
}

#[component]
fn TypographyMutedExample() -> Element {
    rsx! {
        Example { title: "Muted",
            TypographyMuted { "Enter your email address." }
        }
    }
}

/// Plain elements with logical classes, as upstream writes them: the block's classes are physical
/// (`border-l-2`, `ml-6`, `text-left`).
#[component]
fn TypographyRtl() -> Element {
    let p = "leading-7 [&:not(:first-child)]:mt-6";
    let h3 = "mt-8 scroll-m-20 text-2xl font-semibold tracking-tight";
    let th = "border px-4 py-2 text-start font-bold";
    let td = "border px-4 py-2 text-start";
    let tr = "m-0 border-t p-0 even:bg-muted";
    rsx! {
        Example { title: "RTL", class: "h-auto",
            div { dir: "rtl",
                h1 { class: "scroll-m-20 text-4xl font-extrabold tracking-tight text-balance",
                    "فرض الضرائب على الضحك: سجلات ضريبة النكتة"
                }
                p { class: "text-xl leading-7 text-muted-foreground [&:not(:first-child)]:mt-6",
                    "في قديم الزمان، في أرض بعيدة، كان هناك ملك كسول جداً يقضي يومه كله مستلقياً على عرشه. في أحد الأيام، جاءه مستشاروه بمشكلة: المملكة كانت تنفد من المال."
                }
                h2 { class: "mt-10 scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight transition-colors first:mt-0",
                    "خطة الملك"
                }
                p { class: p,
                    "فكر الملك طويلاً وبجد، وأخيراً توصل إلى "
                    a {
                        href: "#",
                        class: "font-medium text-primary underline underline-offset-4",
                        "خطة عبقرية"
                    }
                    ": سيفرض ضريبة على النكات في المملكة."
                }
                blockquote { class: "mt-6 border-s-2 ps-6 italic",
                    "\"في النهاية،\" قال، \"الجميع يستمتع بنكتة جيدة، لذا من العدل أن يدفعوا مقابل هذا الامتياز.\""
                }
                h3 { class: h3, "ضريبة النكتة" }
                p { class: p, "لم يكن رعايا الملك سعداء. تذمروا واشتكوا، لكن الملك كان حازماً:" }
                ul { class: "my-6 ms-6 list-disc [&>li]:mt-2",
                    li { "المستوى الأول من التورية: 5 قطع ذهبية" }
                    li { "المستوى الثاني من النكات: 10 قطع ذهبية" }
                    li { "المستوى الثالث من النكات القصيرة: 20 قطعة ذهبية" }
                }
                p { class: p,
                    "نتيجة لذلك، توقف الناس عن رواية النكات، وغرقت المملكة في الكآبة. لكن كان هناك شخص واحد رفض أن تحبطه حماقة الملك: مهرج البلاط المسمى المازح."
                }
                h3 { class: h3, "ثورة المازح" }
                p { class: p,
                    "بدأ المازح يتسلل إلى القلعة في منتصف الليل ويترك النكات في كل مكان: تحت وسادة الملك، في حسائه، حتى في المرحاض الملكي. كان الملك غاضباً، لكنه لم يستطع إيقاف المازح."
                }
                p { class: p,
                    "وبعد ذلك، في يوم من الأيام، اكتشف سكان المملكة أن النكات التي تركها المازح كانت مضحكة جداً لدرجة أنهم لم يستطيعوا منع أنفسهم من الضحك. وبمجرد أن بدأوا بالضحك، لم يستطيعوا التوقف."
                }
                h3 { class: h3, "ثورة الشعب" }
                p { class: p,
                    "شعر سكان المملكة بالبهجة من الضحك، وبدأوا في رواية النكات والتورية مرة أخرى، وسرعان ما أصبحت المملكة بأكملها جزءاً من النكتة."
                }
                div { class: "my-6 w-full overflow-y-auto",
                    table { class: "w-full",
                        thead {
                            tr { class: tr,
                                th { class: th, "خزينة الملك" }
                                th { class: th, "سعادة الشعب" }
                            }
                        }
                        tbody {
                            tr { class: tr,
                                td { class: td, "فارغة" }
                                td { class: td, "فائضة" }
                            }
                            tr { class: tr,
                                td { class: td, "متواضعة" }
                                td { class: td, "راضٍ" }
                            }
                            tr { class: tr,
                                td { class: td, "ممتلئة" }
                                td { class: td, "منتشٍ" }
                            }
                        }
                    }
                }
                p { class: p,
                    "الملك، عندما رأى مدى سعادة رعاياه، أدرك خطأ طرقه وألغى ضريبة النكتة. أُعلن المازح بطلاً، وعاشت المملكة في سعادة دائمة."
                }
                p { class: p,
                    "مغزى القصة هو: لا تستهن أبداً بقوة الضحك الجيد وكن دائماً حذراً من الأفكار السيئة."
                }
            }
        }
    }
}
