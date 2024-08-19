```toml
[dependencies]
makepad-widgets = {path = "E:/Rust/try/makepad/makepad/rik/makepad/widgets"}
```


```
<template>
  <root id="ui">
    <window id="main_window">
      <view flow="Down" height="All" id="main_view">
        <button :for="(index, value) in btnlist" :text="value" :font_size="btn_size"></button>
        <button :if="flag1" text="True Btn"></button>
        <button else text="False Btn"></button>
      </view>
    </window>
  </root>
</template>

<script>
let btnlist: Vec<String> = vec!["a".to_string(), "b".to_string()];
let flag1 = true;
</script>

<style>
#ui{
  #main_window{
    width: Fill;
    height: Fill;
    flow: Down;
    window_size: 600.0, 800.0;
  }
}
</style>
```