# TCCPlus Gui!

This is TCCPlus but Gui. you need to have TCCplus store in your /Applications/ folder in order for it to work.
<br>
# it crashed!
there's a high chance that the app you're trying to use has Binary `Info.plist` instead of regular text one.
<br>
# FAQ
##### Q: Do i need to have tccplus installed?
- yes! you do need to have tccplus executable inside your `/Applications/` folder! by simply downloading the tccplus executable and drop it into the `/Applcations/` folder!
  
##### Q: Why are you using Tauri or Webview instead of swiftUI or storyboard?
- I don't like Objective C :< and I'm still new to macOS developement so I'm going to keep Swift for later.

<br>

# Building

##### Preperations

You will need:
- Rust
- Node.JS
- Yarn

##### Building

Clone this Repo somewhere and cd into it
Install dependencies:
```
yarn
```

Build in Debug mode:
```
yarn tauri dev
```

Build in Release mode:
```
yarn tauri build
```





