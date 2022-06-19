package net.insprill.customcarmanager.util;

import java.awt.Desktop;
import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.Comparator;
import java.util.stream.Stream;

public class IO {

    public static void deleteDirectory(File dir) throws IOException {
        if (!dir.isDirectory())
            throw new IllegalArgumentException("File must be a directory");

        try (Stream<Path> walk = Files.walk(dir.toPath())) {
            walk.sorted(Comparator.reverseOrder())
                    .map(Path::toFile)
                    .forEach(File::delete);
        }
    }

    public static void moveToTrash(File file) {
        Desktop.getDesktop().moveToTrash(file);
    }

    public static void copyDirectory(File source, File dest) throws IOException {
        if (!source.isDirectory())
            throw new IllegalArgumentException("File must be a directory");
        if (dest.exists())
            throw new IllegalStateException("Destination file already exists");

        try (Stream<Path> stream = Files.walk(source.toPath())) {
            stream.forEach(file -> copy(file, dest.toPath().resolve(source.toPath().relativize(file))));
        }
    }

    private static void copy(Path source, Path dest) {
        try {
            Files.copy(source, dest, StandardCopyOption.REPLACE_EXISTING);
        } catch (Exception e) {
            throw new RuntimeException(e.getMessage(), e);
        }
    }

    private IO() {
        throw new IllegalStateException("Utility class");
    }

}
