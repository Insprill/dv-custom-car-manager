package net.insprill.customcarmanager.util;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.List;

public class Processes {

    /**
     * @return A list of all running processes from the tasklist command.
     * @throws IOException If an IO error occurs.
     */
    private static List<String> getRunningProcesses() throws IOException {
        Process process = Runtime.getRuntime().exec("tasklist.exe");
        try (BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()))) {
            return reader.lines().skip(2).toList(); // The first 2 lines are for formatting
        }
    }

    /**
     * @param processName The name of the process, without an extension (.exe)
     * @return Whether the process is running.
     * @throws IOException If an IO error occurs.
     */
    public static boolean isProcessRunning(String processName) throws IOException {
        for (String process : getRunningProcesses()) {
            String name = process.split("\\.")[0];
            if (name.equalsIgnoreCase(processName))
                return true;
        }
        return false;
    }

    private Processes() {
        throw new IllegalStateException("Utility class");
    }

}
