// 为下列文法构造递归下降语法分析器：
// 1）S -> + S S | - S S | a
// 2）S -> S ( S ) S | ε
// 3）S -> 0 S 1 | 0 1

import java.lang.reflect.Constructor;
import java.lang.reflect.InvocationTargetException;
import java.util.Map;
import java.util.PrimitiveIterator;

// 1）前缀表达式
// S -> + S S | - S S | a
class PrefixNotationParser {
  PrimitiveIterator.OfInt chars;
  int lookahead;

  public PrefixNotationParser(String s) {
    if (s.isEmpty()) {
      return;
    }
    this.chars = s.chars().iterator();
    this.lookahead = this.chars.nextInt();
  }

  public void parse() throws Exception {
    this.expr();
    if (this.lookahead != 0) {
      this.report("syntax error");
    }
  }

  void expr() throws Exception {
    switch (this.lookahead) {
      case '+':
        this.match('+');
        this.expr();
        this.expr();
        break;
      case '-':
        this.match('-');
        this.expr();
        this.expr();
        break;
      case 'a':
        this.match('a');
        break;
      default:
        this.report("syntax error");
    }
  }

  void match(char c) throws Exception {
    if (this.lookahead != c) {
      this.report("syntax error");
    }
    if (this.chars.hasNext()) {
      this.lookahead = this.chars.nextInt();
    } else {
      this.lookahead = 0;
    }
  }

  void report(String reason) throws Exception {
    throw new Exception(reason); // 懒得找合适的 Exception 了
  }
}

// 2）括号对
// S -> S ( S ) S | ε
// 消除左递归
// S -> R
// R -> ( S ) S R | ε
// 就是
// S -> ( S ) S S | ε
//
// 其实这题，直接计数左括号和右括号数的差值数都比递归下降要快
class ParenthesesParser {
  PrimitiveIterator.OfInt chars;
  int lookahead;

  public ParenthesesParser(String s) {
    if (s.isEmpty()) {
      return;
    }
    this.chars = s.chars().iterator();
    this.lookahead = this.chars.nextInt();
  }

  public void parse() throws Exception {
    this.parentheses();
    if (this.lookahead != 0) {
      this.report("syntax error");
    }
  }

  void parentheses() throws Exception {
    if (this.lookahead != '(') {
      return; // S -> epsilon
    }
    this.match('(');
    this.parentheses();
    this.match(')');
    this.parentheses();
    this.parentheses();
  }

  void match(char c) throws Exception {
    if (this.lookahead != c) {
      this.report("syntax error");
    }
    if (this.chars.hasNext()) {
      this.lookahead = this.chars.nextInt();
    } else {
      this.lookahead = 0;
    }
  }

  void report(String reason) throws Exception {
    throw new Exception(reason);
  }
}

// 3）
// S -> 0 S 1 | 0 1
class ZerosOnesParser {
  PrimitiveIterator.OfInt chars;
  int lookahead;

  public ZerosOnesParser(String s) {
    if (s.isEmpty()) {
      return;
    }
    this.chars = s.chars().iterator();
    this.lookahead = this.chars.nextInt();
  }

  public void parse() throws Exception {
    this.zerosOnes();
    if (this.lookahead != 0) {
      this.report("syntax error");
    }
  }

  void zerosOnes() throws Exception {
    this.match('0');
    if (this.lookahead != '1') {
      this.zerosOnes();
    }
    this.match('1');
  }

  void match(char c) throws Exception {
    if (this.lookahead != c) {
      this.report("syntax error");
    }
    if (this.chars.hasNext()) {
      this.lookahead = this.chars.nextInt();
    } else {
      this.lookahead = 0;
    }
  }

  void report(String reason) throws Exception {
    throw new Exception(reason);
  }
}

class Main {
  static Map<String, Class<?>> classes;
  static {
    try {
      classes = Map.of( //
          "1", PrefixNotationParser.class, //
          "2", ParenthesesParser.class, //
          "3", ZerosOnesParser.class);
    } catch (Exception e) {
      throw new Error(e);
    }
  }

  public static void main(String[] args) throws Exception {
    if (args.length == 0) {
      throw new Exception("Please provide class name");
    }
    Class<?> clazz = classes.get(args[0]);
    if (clazz == null) {
      throw new Exception("Unsupported exercise number");
    }
    Constructor<?> ctor = clazz.getDeclaredConstructor(String.class);
    java.io.InputStreamReader is = new java.io.InputStreamReader(System.in);
    java.io.BufferedReader reader = new java.io.BufferedReader(is);
    for (String s = reader.readLine(); s != null; s = reader.readLine()) {
      try {
        Object inst = ctor.newInstance(s);
        clazz.getMethod("parse").invoke(inst);
      } catch (InvocationTargetException e) {
        System.out.println(e.getCause());
      }
    }
  }
}
