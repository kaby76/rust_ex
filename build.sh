# Generated from trgen 0.23.21
set -e
if [ -f transformGrammar.py ]; then python3 transformGrammar.py ; fi

JAR=~/rust-antlr4-tool/tool/target/antlr4-4.8-2-SNAPSHOT-complete.jar
#JAR=/c/Users/Kenne/Downloads/antlr4-4.8-2-SNAPSHOT-complete.jar
java -jar $JAR -encoding utf-8 -Dlanguage=Rust  abbLexer.g4
java -jar $JAR -encoding utf-8 -Dlanguage=Rust  abbParser.g4

cargo b

exit 0
