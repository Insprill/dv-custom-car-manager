package net.insprill.customcarmanager.util;

import java.awt.Toolkit;
import java.awt.datatransfer.Clipboard;
import java.awt.datatransfer.StringSelection;

public class TextHelper {

    /**
     * Gets the stacktrace of a {@link Throwable}, formatted the same as {@link Throwable#printStackTrace()}.
     *
     * @param ex The {@link Throwable} to get the stacktrace of.
     * @return The throwables stacktrace.
     */
    public static String getStacktrace(Throwable ex) {
        StringBuilder sb = new StringBuilder();

        sb.append(ex.getClass().getName()).append(": ").append(ex.getLocalizedMessage());
        sb.append("\n");

        for (StackTraceElement element : ex.getStackTrace()) {
            sb.append(" at ");
            sb.append(element.toString());
            sb.append("\n");
        }
        return sb.toString();
    }

    /**
     * Copies a String of text to the system's clipboard.
     *
     * @param text The text to copy.
     */
    public static void copyToClipboard(String text) {
        StringSelection stringSelection = new StringSelection(text);
        Clipboard clipboard = Toolkit.getDefaultToolkit().getSystemClipboard();
        clipboard.setContents(stringSelection, null);
    }

    private TextHelper() {
        throw new IllegalStateException("Utility class");
    }

}
