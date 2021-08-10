package lexer;

public class Word extends Token {
  public final String lexeme;

  public Word(int t, String s) {
    super(t);
    this.lexeme = new String(s);
  }

  @Override
  public String toString() {
    return "<word " + this.lexeme + ">";
  }
}
