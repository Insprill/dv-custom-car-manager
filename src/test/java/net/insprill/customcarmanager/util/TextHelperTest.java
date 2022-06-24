package net.insprill.customcarmanager.util;

import org.junit.jupiter.api.Test;

import java.awt.Toolkit;
import java.awt.datatransfer.DataFlavor;
import java.awt.datatransfer.UnsupportedFlavorException;
import java.io.IOException;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class TextHelperTest {

    @Test
    void getStacktrace() {
        RuntimeException ex = new RuntimeException("Uh oh");

        String stack = TextHelper.getStacktrace(ex);

        assertTrue(stack.startsWith("java.lang.RuntimeException: Uh oh\n at net.insprill.customcarmanager.util.TextHelperTest.getStacktrace(TextHelperTest.java:"));
    }

    @Test
    void copyToClipboard() throws IOException, UnsupportedFlavorException {
        TextHelper.copyToClipboard("Howdy");

        assertEquals("Howdy", Toolkit.getDefaultToolkit().getSystemClipboard().getContents(null).getTransferData(DataFlavor.stringFlavor));
    }

}
