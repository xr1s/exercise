import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.PushbackReader;
import java.io.Reader;
import java.util.Hashtable;

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

  void skipBlockComment() throws IOException {
    while (this.peek != -1) {
      if (this.peek == '*') {
        this.peek = this.reader.read();
        if (this.peek == '/') {
          this.peek = this.reader.read();
          return;
        }
      }
      this.peek = this.reader.read();
    }
  }

  void skipLineComment() throws IOException {
    while (this.peek != '\n' && this.peek != -1) {
      this.peek = this.reader.read();
    }
  }

  public Token scan() throws IOException {
    for (;; this.peek = this.reader.read()) {
      if (this.peek == ' ' || this.peek == '\t') continue;
      else if (this.peek == '\n') ++line;
      else break;
    }
    if (this.peek == '/') {
      this.peek = this.reader.read();
      if (this.peek == '*') {
        this.peek = this.reader.read();
        this.skipBlockComment();
        return this.scan(); // 尾递归
      }
      if (this.peek == '/') {
        this.peek = this.reader.read();
        this.skipLineComment();
        return this.scan(); // 尾递归
      }
      // 如果不是注释，继续往下走
      // 将会返回一个 / 的 Token
      this.reader.unread(this.peek);
      this.peek = '/';
    }
    if (Character.isDigit(this.peek)) {
      int v = 0;
      do {
        v = 10 * v + Character.digit(this.peek, 10);
        this.peek = this.reader.read();
      } while (Character.isDigit(this.peek));
      return new Num(v);
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
      if (token.tag == Tag.ID || token.tag == Tag.TRUE || token.tag == Tag.FALSE) {
        var word = (Word)token;
        System.out.printf("Word: %s\n", word.lexeme);
        continue;
      }
      if (token.tag == Tag.NUM) {
        var num = (Num)token;
        System.out.printf("Num: %d\n", num.value);
        continue;
      }
      System.out.printf("Char: %c\n", (char)token.tag);
    }
  }
}
