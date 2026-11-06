//! Accordion examples, ported from shadcn/ui `examples/base/accordion-*.tsx`.

use dioxus::prelude::*;

use super::Example;
use crate::components::ui::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger,
};
use crate::components::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

const PREVIEW: &str = "*:data-[slot=accordion]:max-w-sm";

#[component]
pub fn Examples() -> Element {
    rsx! {
        AccordionDemo {}
        AccordionBasic {}
        AccordionMultiple {}
        AccordionDisabled {}
        AccordionBorders {}
        AccordionCard {}
        AccordionRtl {}
    }
}

struct Item {
    value: &'static str,
    trigger: &'static str,
    content: &'static str,
}

#[component]
fn AccordionDemo() -> Element {
    rsx! {
        Example { title: "Demo", class: "{PREVIEW} h-[300px]",
            Accordion { default_value: vec!["shipping".to_string()], class: "max-w-lg",
                AccordionItem { value: "shipping",
                    AccordionTrigger { "What are your shipping options?" }
                    AccordionContent {
                        "We offer standard (5-7 days), express (2-3 days), and overnight shipping. Free shipping on international orders."
                    }
                }
                AccordionItem { value: "returns",
                    AccordionTrigger { "What is your return policy?" }
                    AccordionContent {
                        "Returns accepted within 30 days. Items must be unused and in original packaging. Refunds processed within 5-7 business days."
                    }
                }
                AccordionItem { value: "support",
                    AccordionTrigger { "How can I contact customer support?" }
                    AccordionContent {
                        "Reach us via email, live chat, or phone. We respond within 24 hours during business days."
                    }
                }
            }
        }
    }
}

#[component]
fn AccordionBasic() -> Element {
    let items = [
        Item {
            value: "item-1",
            trigger: "How do I reset my password?",
            content: "Click on 'Forgot Password' on the login page, enter your email address, and we'll send you a link to reset your password. The link will expire in 24 hours.",
        },
        Item {
            value: "item-2",
            trigger: "Can I change my subscription plan?",
            content: "Yes, you can upgrade or downgrade your plan at any time from your account settings. Changes will be reflected in your next billing cycle.",
        },
        Item {
            value: "item-3",
            trigger: "What payment methods do you accept?",
            content: "We accept all major credit cards, PayPal, and bank transfers. All payments are processed securely through our payment partners.",
        },
    ];
    rsx! {
        Example { title: "Basic", class: "{PREVIEW} h-[300px]",
            Accordion { default_value: vec!["item-1".to_string()], class: "max-w-lg",
                for item in items {
                    AccordionItem { key: "{item.value}", value: item.value,
                        AccordionTrigger { "{item.trigger}" }
                        AccordionContent { "{item.content}" }
                    }
                }
            }
        }
    }
}

#[component]
fn AccordionMultiple() -> Element {
    let items = [
        Item {
            value: "notifications",
            trigger: "Notification Settings",
            content: "Manage how you receive notifications. You can enable email alerts for updates or push notifications for mobile devices.",
        },
        Item {
            value: "privacy",
            trigger: "Privacy & Security",
            content: "Control your privacy settings and security preferences. Enable two-factor authentication, manage connected devices, review active sessions, and configure data sharing preferences. You can also download your data or delete your account.",
        },
        Item {
            value: "billing",
            trigger: "Billing & Subscription",
            content: "View your current plan, payment history, and upcoming invoices. Update your payment method, change your subscription tier, or cancel your subscription.",
        },
    ];
    rsx! {
        Example { title: "Multiple", class: "{PREVIEW} h-[450px]",
            Accordion {
                multiple: true,
                class: "max-w-lg",
                default_value: vec!["notifications".to_string()],
                for item in items {
                    AccordionItem { key: "{item.value}", value: item.value,
                        AccordionTrigger { "{item.trigger}" }
                        AccordionContent { "{item.content}" }
                    }
                }
            }
        }
    }
}

#[component]
fn AccordionDisabled() -> Element {
    rsx! {
        Example { title: "Disabled", class: "{PREVIEW} h-[300px]",
            Accordion { class: "w-full",
                AccordionItem { value: "item-1",
                    AccordionTrigger { "Can I access my account history?" }
                    AccordionContent {
                        "Yes, you can view your complete account history including all transactions, plan changes, and support tickets in the Account History section of your dashboard."
                    }
                }
                AccordionItem { value: "item-2", disabled: true,
                    AccordionTrigger { "Premium feature information" }
                    AccordionContent {
                        "This section contains information about premium features. Upgrade your plan to access this content."
                    }
                }
                AccordionItem { value: "item-3",
                    AccordionTrigger { "How do I update my email address?" }
                    AccordionContent {
                        "You can update your email address in your account settings. You'll receive a verification email at your new address to confirm the change."
                    }
                }
            }
        }
    }
}

#[component]
fn AccordionBorders() -> Element {
    let items = [
        Item {
            value: "billing",
            trigger: "How does billing work?",
            content: "We offer monthly and annual subscription plans. Billing is charged at the beginning of each cycle, and you can cancel anytime. All plans include automatic backups, 24/7 support, and unlimited team members.",
        },
        Item {
            value: "security",
            trigger: "Is my data secure?",
            content: "Yes. We use end-to-end encryption, SOC 2 Type II compliance, and regular third-party security audits. All data is encrypted at rest and in transit using industry-standard protocols.",
        },
        Item {
            value: "integration",
            trigger: "What integrations do you support?",
            content: "We integrate with 500+ popular tools including Slack, Zapier, Salesforce, HubSpot, and more. You can also build custom integrations using our REST API and webhooks.",
        },
    ];
    rsx! {
        Example { title: "Borders", class: "{PREVIEW} h-[300px]",
            Accordion {
                class: "max-w-lg rounded-lg border",
                default_value: vec!["billing".to_string()],
                for item in items {
                    AccordionItem {
                        key: "{item.value}",
                        value: item.value,
                        class: "border-b px-4 last:border-b-0",
                        AccordionTrigger { "{item.trigger}" }
                        AccordionContent { "{item.content}" }
                    }
                }
            }
        }
    }
}

#[component]
fn AccordionCard() -> Element {
    let items = [
        Item {
            value: "plans",
            trigger: "What subscription plans do you offer?",
            content: "We offer three subscription tiers: Starter ($9/month), Professional ($29/month), and Enterprise ($99/month). Each plan includes increasing storage limits, API access, priority support, and team collaboration features.",
        },
        Item {
            value: "billing",
            trigger: "How does billing work?",
            content: "Billing occurs automatically at the start of each billing cycle. We accept all major credit cards, PayPal, and ACH transfers for enterprise customers. You'll receive an invoice via email after each payment.",
        },
        Item {
            value: "cancel",
            trigger: "How do I cancel my subscription?",
            content: "You can cancel your subscription anytime from your account settings. There are no cancellation fees or penalties. Your access will continue until the end of your current billing period.",
        },
    ];
    rsx! {
        Example { title: "Card", class: "{PREVIEW} h-[435px]",
            Card { class: "w-full max-w-sm",
                CardHeader {
                    CardTitle { "Subscription & Billing" }
                    CardDescription {
                        "Common questions about your account, plans, payments and cancellations."
                    }
                }
                CardContent {
                    Accordion { default_value: vec!["plans".to_string()],
                        for item in items {
                            AccordionItem { key: "{item.value}", value: item.value,
                                AccordionTrigger { "{item.trigger}" }
                                AccordionContent { "{item.content}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AccordionRtl() -> Element {
    let items = [
        Item {
            value: "item-1",
            trigger: "كيف يمكنني إعادة تعيين كلمة المرور؟",
            content: "انقر على 'نسيت كلمة المرور' في صفحة تسجيل الدخول، أدخل عنوان بريدك الإلكتروني، وسنرسل لك رابطًا لإعادة تعيين كلمة المرور. سينتهي صلاحية الرابط خلال 24 ساعة.",
        },
        Item {
            value: "item-2",
            trigger: "هل يمكنني تغيير خطة الاشتراك الخاصة بي؟",
            content: "نعم، يمكنك ترقية أو تخفيض خطتك في أي وقت من إعدادات حسابك. ستظهر التغييرات في دورة الفوترة التالية.",
        },
        Item {
            value: "item-3",
            trigger: "ما هي طرق الدفع التي تقبلونها؟",
            content: "نقبل جميع بطاقات الائتمان الرئيسية و PayPal والتحويلات المصرفية. تتم معالجة جميع المدفوعات بأمان من خلال شركاء الدفع لدينا.",
        },
    ];
    rsx! {
        Example { title: "RTL", class: "{PREVIEW}",
            Accordion {
                default_value: vec!["item-1".to_string()],
                class: "max-w-md",
                dir: "rtl",
                for item in items {
                    AccordionItem { key: "{item.value}", value: item.value,
                        AccordionTrigger { "{item.trigger}" }
                        AccordionContent { "{item.content}" }
                    }
                }
            }
        }
    }
}
