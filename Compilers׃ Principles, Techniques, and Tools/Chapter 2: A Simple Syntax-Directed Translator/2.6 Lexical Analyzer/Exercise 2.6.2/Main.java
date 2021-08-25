import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.PushbackReader;
import java.io.Reader;
import java.util.Hashtable;

import lexer.Int;
import lexer.Rel;
import lexer.Tag;
import lexer.Token;
import lexer.Word;

class Lexer {
  public int line = 1;
  private int peek = ' ';
  private Hashtable<String, Word> words = new Hashtable<String, Word>();
  PushbackReader reader;

  void reserve(Word word) {
    this.words.put(word.lexeme, word);
  }

  public Lexer() {
    this(System.in);
  }

  public Lexer(InputStream is) {
    this(new InputStreamReader(is));
  }

  public Lexer(Reader reader) {
    this.reserve(new Word(Tag.TRUE, "true"));
    this.reserve(new Word(Tag.FALSE, "false"));
    this.reader = new PushbackReader(reader);
  }

  // 可能会返回包括 <, <=, !, !=, =, ==, >, >= 八种 Token
  Token scanRel() throws IOException {
    int prev = this.peek;
    this.peek = this.reader.read();
    int next = 0;
    if (this.peek == '=') {
      next = this.peek;
      this.peek = this.reader.read();
    }
    switch (prev * 256 + next) {
      case '<' * 256:
        return Rel.LT;
      case '<' * 256 + '=':
        return Rel.LE;
      case '=' * 256:
        return new Token('=');
      case '=' * 256 + '=':
        return Rel.EQ;
      case '!' * 256:
        return new Token('!');
      case '!' * 256 + '=':
        return Rel.NE;
      case '>' * 256:
        return Rel.GT;
      case '>' * 256 + '=':
        return Rel.GE;
    }
    // unreacheable
    return new Token(-1);
  }

  public Token scan() throws IOException {
    for (;; this.peek = this.reader.read()) {
      if (this.peek == ' ' || this.peek == '\t') continue;
      else if (this.peek == '\n') ++line;
      else break;
    }
    if ("<!=>".indexOf(this.peek) != -1) {
      return this.scanRel();
    }
    if (Character.isDigit(this.peek)) {
      int v = 0;
      do {
        v = 10 * v + Character.digit(this.peek, 10);
        this.peek = this.reader.read();
      } while (Character.isDigit(this.peek));
      return new Int(v);
    }
    if (Character.isLetter(this.peek)) {
      StringBuilder b = new StringBuilder();
      do {
        b.append((char)this.peek);
        this.peek = this.reader.read();
      } while (Character.isLetterOrDigit(this.peek));
      String s = b.toString();
      Word w = words.get(s);
      if (w != null) return w;
      w = new Word(Tag.ID, s);
      words.put(s, w);
      return w;
    }
    Token t = new Token(this.peek);
    this.peek = ' ';
    return t;
  }
}

public class Main {
  public static void main(String[] args) throws IOException {
    var lexer = new Lexer();
    for (var token = lexer.scan(); token.tag != -1; token = lexer.scan()) {
      System.out.println(token);
    }
  }
}
