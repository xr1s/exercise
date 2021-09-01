package lexer;

public class Int extends Token {
  public final int value;

  public Int(int v) {
    super(Tag.INT);
    this.value = v;
  }

  @Override
  public String toString() {
    return "<int " + value + ">";
  }
}
