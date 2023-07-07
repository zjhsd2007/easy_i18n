### easy i18n
#### A easy i18n tool
#### Example
```rust
use easy_i18n::{self, i18n, I18N};
use std::path::Path;

// set source
easy_i18n::set_source(Path::new("./src/source"));

// set lang
easy_i18n::set_lang("EN");

i18n!("这是一个测试"); // This is a test

// Sometimes, the same text has different translation results in different contexts. At this time, we can set different namespaces
i18n!("这是一个测试", ns="namespace1"); // This is a test, but it is different

// If there is a dynamic value in the text, we can use %1, %2, %3.. as a placeholder, where the number represents the position of the dynamic value
i18n!("他的成绩是，语文：%1, 数学：%2", 88, 100); // His grades are Chinese: 88, Mathematics: 100

// If you have different translation results in other contexts, you can set the namespace
i18n!("他的成绩是，语文：%1, 数学：%2", ns="namespace1", 88, 100); // His grades are Chinese: 88, Mathematics: 100, and the test is not bad.
```

source is a directory that contains some translated text and corresponding translation results <br/>
```
your_project
  |--src
    |--source
      |-- cn.json
      |-- en.json
      |-- de.json
```
The format of each json file is, `common` is required. and optional `namespace` fields represent a different context
```json
{
  "common": {
    "这是一个测试": "This is a test",
    ...
  }
}
```
If the same text has different translation results in different contexts, you can add  `namespace` in the json file, for example, the content of `en.json` is:
```json
{
  "common": {
    "这是一个测试": "This is a test"
  },
  "namespace1": {
    "这是一个测试": "This is a test, but it is different"
  },
  "other_ns": {
    "这是一个测试": "This is a test, haha!"
  }
}
```
Then just pass in the corresponding `namespace`, for example
```rust
i18n!("这是一个测试"); // This is a test
i18n!("这是一个测试", ns="namespace1"); // This is a test, but it is different
i18n!("这是一个测试", ns="other_ns"); // This is a test, haha!
```

If the text contains dynamic values, `%1`, `%2` can be used as placeholders, where 1, 2 indicate the position of the dynamic value, starting from 1, for example, the content of `en.json` is :
```json
{
  "common": {
    "这是一个测试": "This is a test",
    "他的成绩是，语文：%1, 数学：%2": "His grades are Chinese: %1, Mathematics: %2"
  }
}
```
usage：
```rust
i18n!("他的成绩是，语文：%1, 数学：%2", 88, 100); // His grades are Chinese: 88, Mathematics: 100
```

When the dynamic value and `namesapce` exist at the same time, `namespace` is placed in front of the dynamic value, for example, the content of `en.json` is:
```json
{
  "common": {
    "这是一个测试": "This is a test",
    "他的成绩是，语文：%1, 数学：%2": "His grades are Chinese: %1, Mathematics: %2"
  },
  "ns":{
    "他的成绩是，语文：%1, 数学：%2": "His grades are Chinese: %1, Mathematics: %2, and the test is not bad."
  }
}
```
```rust
i18n!("他的成绩是，语文：%1, 数学：%2", ns="ns", 88, 100); // His grades are Chinese: 88, Mathematics: 100, and the test is not bad.
```