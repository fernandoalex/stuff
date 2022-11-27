class Switch {
    public static void main (String[] args) {
	int value = 1;

	switch (value) {
		case 1:
			System.out.println("Value was 1");
			break;
		case 2:
			System.out.println("Value was 1");
			break;
		case 3: case 4: case 5: { 
			System.out.println("Value was 3,4 or a 5"); 
			break;
		}
		default: {
			System.out.println("default");
			break;
		}
	}

	// "Enhanced" switch
	// - no fall through, so no breaks;
	switch (value) {
		case 1 -> System.out.println("Value was 1");
		case 2 -> System.out.println("Value was 1");
		case 3, 4, 5 -> { 
			System.out.println("Value was 3,4 or a 5"); 
		}
		default -> System.out.println("default");
	}

	String month = "APRIL";

	System.out.println(getQuarter(month));
    }

    public static String getQuarter(String month) {
	return switch (month) {
		case "JANUARY", "FEBRUARY", "MARCH" -> "1st";
		case "APRIL", "MAY", "JUNE" -> "2nd";
		case "JULY", "AUGUST", "SEPTEMBER" -> "3rd";
		case "OCTOBER", "NOVEMBER", "DECEMBER" -> "4th";
		default -> {
		    String badResponse = month + " is bad";
		    // return badResponse; // would failt here
		    yield badResponse;
		}
	};
    }
}
