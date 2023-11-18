use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use paste::paste;

macro_rules! java_fn {
  ($name:ident($( $( $v:ident: $t:ty ),+ )?) $code:block) => {
    java_fn!($name(_env $(, $( $v: $t )+ )?) -> () $code);
  };
  ($name:ident( $( $( $v:ident: $t:ty ),+ )? ) -> $ret:ty $code:block) => {
    java_fn!($name(_env $(, $( $v: $t )+ )?) -> $ret $code);
  };
  ($name:ident($env:ident $(, $( $v:ident: $t:ty ),+ )? ) $code:block) => {
    java_fn!($name($env $(, $( $v: $t )+ )?) -> () $code);
  };
  ($name:ident($env:ident $(, $( $v:ident: $t:ty ),+ )?) -> $ret:ty $code:block) => {
    paste! {
      #[no_mangle]
      pub extern "system" fn [<Java_steptech_jminesweeper_UiBindings_ $name>](
        #[allow(unused_mut)]
        mut $env: JNIEnv,
        _class: JClass,
        $( $( $v: $t )+ )?
      ) -> $ret $code
    }
  };
}

java_fn!(helloWorld() {
  println!("Hello Java from Rust!");
});

java_fn!(hello(env, input: JString) -> jstring {
  let input: String = env.get_string(&input).expect("can't convert JString to String").into();
  let output = env.new_string(format!("Hello {}!", input)).expect("can't create JString from String");
  output.into_raw()
});
