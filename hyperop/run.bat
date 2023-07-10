@ECHO OFF 
javac Hyperop.java -d out
cd out
@ECHO ON
java Hyperop
@ECHO OFF
cd ..