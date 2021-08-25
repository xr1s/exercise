package lexer;

public class Rel extends Token {
  private final static int lt = 0;
  private final static int le = 1;
  private final static int eq = 2;
  private final static int ne = 3;
  private final static int gt = 4;
  private final static int ge = 5;

  public final static Rel LT = new Rel(Rel.lt);
  public final static Rel LE = new Rel(Rel.le);
  public final static Rel EQ = new Rel(Rel.eq);
  public final static Rel NE = new Rel(Rel.ne);
  public final static Rel GT = new Rel(Rel.gt);
  public final static Rel GE = new Rel(Rel.ge);
  public int rel;

  public Rel(int rel) {
    super(Tag.REL);
    this.rel = rel;
  }

  @Override
  public String toString() {
    switch (this.rel) {
      case Rel.lt:
        return "<rel \"<\">";
      case Rel.le:
        return "<rel \"<=\">";
      case Rel.eq:
        return "<rel ==>";
      case Rel.ne:
        return "<rel !=>";
      case Rel.gt:
        return "<rel \">\">";
      case Rel.ge:
        return "<rel \">=\">";
    }
    return "";
  }
}
