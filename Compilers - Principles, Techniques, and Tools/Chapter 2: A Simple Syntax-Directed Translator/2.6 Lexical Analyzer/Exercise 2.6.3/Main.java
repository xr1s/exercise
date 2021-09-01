import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.PushbackReader;
import java.io.Reader;
import java.util.Hashtable;

import lexer.Tag;
import lexer.Word;
import lexer.Token;
import lexer.Int;
import lexer.Float;

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

  public Token scanNum() throws IOException {
    boolean hasDot = false;
    StringBuilder builder = new StringBuilder();
    do {
      hasDot |= this.peek == '.';
      builder.append((char)this.peek);
      this.peek = this.reader.read();
      // 一个数字中不能出现两个小数点
    } while (!hasDot && this.peek == '.' || Character.isDigit(this.peek));
    if (!hasDot) return new Int(Integer.parseInt(builder.toString()));
    else return new Float(Double.parseDouble(builder.toString()));
  }

  public Token scan() throws IOException {
    for (;; this.peek = this.reader.read()) {
      if (this.peek == ' ' || this.peek == '\t') continue;
      else if (this.peek == '\n') ++line;
      else break;
    }
    if (Character.isDigit(this.peek)) {
      return this.scanNum();
    }
    if (this.peek == '.') {
      int num = this.reader.read();
      this.reader.unread(num);
      if (Character.isDigit(num)) {
        return this.scanNum();
      }
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
