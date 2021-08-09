class Tag {
  public final static int NUM = 256, ID = 257, TRUE = 258, FALSE = 259;
}

public class Token {
  public final int tag;

  public Token(int t) {
    this.tag = t;
  }
}

class Num extends Token {
  public final int value;

  public Num(int v) {
    super(Tag.NUM);
    this.value = v;
  }
}

class Word extends Token {
  public final String lexeme;

  public Word(int t, String s) {
    super(t);
    this.lexeme = new String(s);
  }
}
