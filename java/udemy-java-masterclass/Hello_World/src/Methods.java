public class Methods {

	public static void main (String[] args) {
		int newScore = calculateScore("name1", 10);
		System.out.println("New score " + newScore);

		calculateScore(75);
		calculateScore();
	}

	// Overloaded methods
	public static int calculateScore(String playerName, int score) {
		System.out.println("Player " + playerName + " scored " + score);
		return score * 1000;
	}

	// there are no default values for params in java so using overload is a way around that
	public static int calculateScore(int score) {
		return calculateScore("Anonymous", score);
	}

	public static void calculateScore() {
		System.out.println("no player name");
	}
	// end Overloaded methods
}
