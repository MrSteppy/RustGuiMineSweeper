# generates C header files for the UiBindings class
javac -h . -d out src/main/java/steptech/jminesweeper/UiBindings.java
# remove generated class files
rm -r out