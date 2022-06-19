package net.insprill.customcarmanager.util;

import java.awt.Toolkit;
import java.awt.datatransfer.Clipboard;
import java.awt.datatransfer.StringSelection;

public class TextHelper {

    public static String getStacktrace(Throwable ex) {
        StringBuilder sb = new StringBuilder();

        sb.append(ex.getClass().getName()).append(": ").append(ex.getLocalizedMessage());
        sb.append("\n");

        for (StackTraceElement element : ex.getStackTrace()) {
            sb.append(element.toString());
            sb.append("\n");
        }
        return sb.toString();
    }

    public static void copyToClipboard(String text) {
        StringSelection stringSelection = new StringSelection(text);
        Clipboard clipboard = Toolkit.getDefaultToolkit().getSystemClipboard();
        clipboard.setContents(stringSelection, null);
    }

    private TextHelper() {
        throw new IllegalStateException("Utility class");
    }

}
