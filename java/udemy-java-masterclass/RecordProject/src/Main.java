class Main {
	public static void main (String[] args) {
		for (int i = 1; i <= 5; i++) {
			Student s = new Student(
					"2900" + i,
					switch (i) {
						case 1 -> "Mary";
						case 2 -> "Carol";
						case 3 -> "Tim";
						case 4 -> "Happy";
						case 5 -> "Lisa";
							default -> "Anonymous";
					},
					"05/01/1990",
					"Java"
					);
			System.out.println(s);

			// when using "records" we do not prefix getter with get
			// also records are meant to be imutable so there are no "setters"
			System.out.println(s.name()); 
		}

	}
}
