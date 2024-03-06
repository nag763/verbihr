use crate::i18n::{Locale, TranslationMap};

const LOCALES : [Locale; 7] = [
	Locale::new("en", "English", vec![
    "en",
    "en_UK",
    "en_US",
], true, TranslationMap(
    {
        "end_perfect": "You didn't do any mistake, it seems like German verbs have no secrets for you anymore!",
        "end_mistakes": "It seems like you did some mistakes, but no worry, it is part of the learning process.",
        "stop_here": "Stop here",
        "give_up": "Give up",
        "intro": "This application is designed to assist in the learning of German irregular verbs, providing a valuable resource for mastering this essential aspect of the German language.",
        "restart": "Restart",
        "start": "Start",
        "end_reached": "You reached the end",
        "error_number": "Number of errors",
        "help_modal_title": "Help",
        "footer": "This work is licensed under a Creative Commons Attribution-NonCommercial 4.0 License. Feel free to share and use for non-profit purposes with proper attribution.",
        "all_rights_reserved": "All rights reserved.",
        "validate": "Validate",
        "clear_inputs": "Clear",
        "keybindings": "Keybindings",
    },
)),
	Locale::new("fr", "Français", vec![
    "fr",
    "fr_FR",
    "fr_BE",
], false, TranslationMap(
    {
        "end_perfect": "Vous n'avez commis aucune erreur, il semble que les verbes allemands n'aient plus de secrets pour vous !",
        "footer": "Ce travail est sous licence Creative Commons Attribution-NonCommercial 4.0 International. N'hésitez pas à le partager et à l'utiliser à des fins non lucratives avec une attribution appropriée.",
        "clear_inputs": "Nettoyer les champs",
        "intro": "Cette application est conçue pour aider à l'apprentissage des verbes irréguliers allemands, fournissant une ressource précieuse pour maîtriser cet aspect essentiel de la langue allemande.",
        "keybindings": "Raccourcis clavier",
        "field_error": "Ce champ n'est pas correct",
        "end_mistakes": "Il semble que vous ayez fait quelques erreurs, mais ne vous inquiétez pas, cela fait partie du processus d'apprentissage.",
        "start": "Démarrer",
        "stop_here": "S'arrêter",
        "give_up": "Abandonner",
        "help_modal_title": "Aide",
        "all_rights_reserved": "Tous droits réservés.",
        "validate": "Valider",
        "end_reached": "Vous avez atteint la fin",
        "error_number": "Nombre d'erreurs",
        "restart": "Recommencer",
    },
)),
	Locale::new("de", "Deutsches", vec![
    "de",
    "de_DE",
    "de_AT",
    "de_CH",
], false, TranslationMap(
    {
        "all_rights_reserved": "Alle Rechte vorbehalten.",
        "help_modal_title": "Hilfe",
        "give_up": "Aufgeben",
        "end_perfect": "Sie haben keinen Fehler gemacht, es sieht so aus, als hätten deutsche Verben keine Geheimnisse mehr für Sie!",
        "keybindings": "Tastenkombinationen",
        "field_error": "Das Feld ist nicht korrekt",
        "stop_here": "Hier aufhalten",
        "end_mistakes": "Es scheint, dass Sie einige Fehler gemacht haben, aber keine Sorge, das gehört zum Lernprozess dazu.",
        "restart": "Neustart",
        "error_number": "Anzahl der Fehler",
        "intro": "Dieses Anwendungsfenster ist speziell da, um Ihnen bei der Lernung der unregelmäßigen deutschen Verben zu helfen und eine wertvolle Ressource für das Meistern dieses essentiellen Aspektes der deutschen Sprache zu sein.",
        "clear_inputs": "Leere Eingaben löschen",
        "footer": "Das Anwendungsfenster ist unter der Creative Commons Namensnennung 4.0 Lizenz bereitgestellt. Freuet euch an, es zu teilen und für non-profit-Zwecke zu verwenden, sofern Sie die angemessenen Angaben machen.",
        "end_reached": "Sie haben das Ende erreicht",
        "start": "Starten",
        "validate": "Überprüfen",
    },
)),
	Locale::new("es", "Español", vec![
    "es",
    "es_ES",
    "es_MX",
], false, TranslationMap(
    {
        "all_rights_reserved": "Todos los derechos reservados.",
        "restart": "Reiniciar",
        "end_perfect": "No cometiste ningún error, ¡parece que los verbos alemanes no tienen secretos para ti!",
        "validate": "Validar",
        "end_reached": "Has llegado al final",
        "intro": "Esta aplicación está diseñada para ayudar en el aprendizaje de los verbos irregulares alemanes, proporcionando un recurso valioso para dominar este aspecto esencial del idioma alemán.",
        "error_number": "Número de errores",
        "keybindings": "Combinaciones de teclas",
        "footer": "Este trabajo está licenciado bajo una Licencia Creative Commons Atribución-NoComercial 4.0. Siéntase libre de compartirlo y usarlo con fines no comerciales con la atribución adecuada.",
        "help_modal_title": "Ayuda",
        "start": "Comenzar",
        "stop_here": "Detenerse",
        "clear_inputs": "Limpiar",
        "give_up": "Rendirse",
        "end_mistakes": "Parece que cometiste algunos errores, pero no te preocupes, es parte del proceso de aprendizaje.",
    },
)),
	Locale::new("ru", "Русский", vec![
    "ru",
    "ru_RU",
], false, TranslationMap(
    {
        "start": "Начать",
        "stop_here": "Остановиться",
        "field_error": "Это поле заполнено неверно",
        "intro": "Это приложение разработано для помощи в изучении неправильных глаголов немецкого языка, предоставляя ценный ресурс для освоения этого важного аспекта немецкого языка.",
        "end_reached": "Вы достигли конца",
        "footer": "Эта работа лицензирована по лицензии Creative Commons Attribution-NonCommercial 4.0. Не стесняйтесь делиться и использовать ее в некоммерческих целях с правильным указанием авторства.",
        "clear_inputs": "Очистить",
        "all_rights_reserved": "Все права защищены.",
        "end_mistakes": "Похоже, что вы допустили несколько ошибок, но не беспокойтесь, это часть процесса обучения.",
        "validate": "Подтвердить",
        "restart": "Перезапустить",
        "help_modal_title": "Помощь",
        "keybindings": "Горячие клавиши",
        "give_up": "Сдаться",
        "end_perfect": "Вы не совершили ни одной ошибки, похоже, что немецкие глаголы больше не являются для вас тайной!",
        "error_number": "Количество ошибок",
    },
)),
	Locale::new("ja", "日本語", vec![
    "ja",
    "ja_JP",
], false, TranslationMap(
    {
        "intro": "このアプリケーションは、ドイツ語の不規則な動詞の学習を支援するために設計され、ドイツ語の言語のこの重要な側面をマスターするための貴重なリソースを提供します。",
        "end_mistakes": "いくつかのミスがあるようですが、心配いりません。これは学習プロセスの一部です。",
        "clear_inputs": "クリア",
        "validate": "確認",
        "keybindings": "キーバインディング",
        "error_number": "エラーの数",
        "footer": "この作業はクリエイティブコモンズ表示-非営利4.0ライセンスの下でライセンスされています。適切な帰属をして非営利目的で共有および使用してください。",
        "end_reached": "終了しました",
        "end_perfect": "どの間違いもありません。ドイツの動詞はもうあなたには秘密がありません！",
        "all_rights_reserved": "全著作権所有。",
        "stop_here": "ここで停止",
        "restart": "再起動",
        "give_up": "諦める",
        "start": "開始",
        "help_modal_title": "ヘルプ",
    },
)),
	Locale::new("zh", "中文", vec![
    "zh",
    "zh_CN",
    "zh_TW",
], false, TranslationMap(
    {
        "end_perfect": "你没有犯任何错误，看起来德语动词对你来说已经没有秘密了！",
        "footer": "本作品根据知识共享署名-非商业4.0许可证授权。请随时在适当归属的情况下共享和用于非营利目的。",
        "error_number": "错误数量",
        "clear_inputs": "清除",
        "all_rights_reserved": "版权所有。",
        "stop_here": "到此为止",
        "keybindings": "快捷键",
        "give_up": "放弃",
        "end_mistakes": "似乎你犯了一些错误，但别担心，这是学习过程的一部分。",
        "intro": "本应用程序旨在协助学习德语不规则动词，为掌握德语语言的这一重要方面提供了宝贵的资源。",
        "help_modal_title": "帮助",
        "restart": "重新开始",
        "end_reached": "你到达了尽头",
        "validate": "验证",
        "start": "开始",
    },
)),
];
