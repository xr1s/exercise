package lexer;

public class Float extends Token {
  public final double value;

  public Float(double v) {
    super(Tag.FLT);
    this.value = v;
  }

  @Override
  public String toString() {
    return "<float " + value + ">";
  }
}
