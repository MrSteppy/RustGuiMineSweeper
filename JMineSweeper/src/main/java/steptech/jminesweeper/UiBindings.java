package steptech.jminesweeper;

import java.net.URL;
import java.util.Objects;

public class UiBindings {

  static {
    String libName = "rust_ui";
    String resourceName = System.mapLibraryName(libName);
    URL resource = UiBindings.class.getClassLoader().getResource(resourceName);
    Objects.requireNonNull(resource, "missing resource: " + resourceName);
    String libPath = resource.getFile();

    System.load(libPath);
  }

  public static native void helloWorld();

  public static native String hello(String input);
}
