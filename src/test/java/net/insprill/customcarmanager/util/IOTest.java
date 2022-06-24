package net.insprill.customcarmanager.util;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.Test;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Comparator;
import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class IOTest {

    @AfterEach
    void teardown() throws IOException {
        try (Stream<Path> walk = Files.walk(newFile("test").toPath())) {
            walk.sorted(Comparator.reverseOrder())
                    .map(Path::toFile)
                    .forEach(File::delete);
        }
    }

    @Test
    void deleteDirectory_NoChildren_Deletes() throws IOException {
        File dir = newFile("test/directory");
        assertTrue(dir.mkdirs());

        IO.deleteDirectory(dir);

        assertFalse(dir.exists());
    }

    @Test
    void deleteDirectory_NestedFiles_Deletes() throws IOException {
        File dir = newFile("test/directory");
        assertTrue(dir.mkdirs());
        File file = newFile(dir, "file.txt");
        assertTrue(file.createNewFile());

        IO.deleteDirectory(dir);

        assertFalse(dir.exists());
        assertFalse(file.exists());
    }

    @Test
    void deleteDirectory_NestedDirectory_Deletes() throws IOException {
        File dir = newFile("test/directory");
        assertTrue(dir.mkdirs());
        File dir2 = newFile(dir, "dir2");
        assertTrue(dir2.mkdirs());

        IO.deleteDirectory(dir);

        assertFalse(dir.exists());
        assertFalse(dir2.exists());
    }

    @Test
    void deleteDirectory_NestedDirectoryWithFiles_Deletes() throws IOException {
        File dir = newFile("test/directory");
        assertTrue(dir.mkdirs());
        File dir2 = newFile(dir, "dir2");
        assertTrue(dir2.mkdirs());
        File file = newFile(dir2, "file.txt");
        assertTrue(file.createNewFile());

        IO.deleteDirectory(dir);

        assertFalse(dir.exists());
        assertFalse(dir2.exists());
        assertFalse(file.exists());
    }

    @Test
    void moveToTrash_MovesToTrash() {
        File dir = newFile("test/directory");
        assertTrue(dir.mkdirs());

        IO.moveToTrash(dir);

        assertFalse(dir.exists());
    }

    @Test
    void copyDirectory_NoChildren_Deletes() throws IOException {
        File src = newFile("test/src");
        assertTrue(src.mkdirs());
        File dest = newFile("test/dest");

        IO.copyDirectory(src, dest);

        assertTrue(dest.exists());
        assertEquals(0, dest.listFiles().length);
    }

    @Test
    void copyDirectory_NestedFiles_Deletes() throws IOException {
        File dir = newFile("test/directory");
        assertTrue(dir.mkdirs());
        File file = newFile(dir, "file.txt");
        assertTrue(file.createNewFile());
        File dest = newFile("test/dest");

        IO.copyDirectory(dir, dest);

        assertTrue(dir.exists());
        assertTrue(file.exists());
        assertTrue(newFile(dest, file.getName()).exists());
    }

    @Test
    void copyDirectory_NestedDirectory_Deletes() throws IOException {
        File dir = newFile("test/directory");
        assertTrue(dir.mkdirs());
        File dir2 = newFile(dir, "dir2");
        assertTrue(dir2.mkdirs());
        File dest = newFile("test/dest");

        IO.copyDirectory(dir, dest);

        assertTrue(dir.exists());
        assertTrue(dir2.exists());
        assertTrue(newFile(dest, dir2.getName()).exists());
    }

    @Test
    void copyDirectory_NestedDirectoryWithFiles_Deletes() throws IOException {
        File dir = newFile("test/directory");
        assertTrue(dir.mkdirs());
        File dir2 = newFile(dir, "dir2");
        assertTrue(dir2.mkdirs());
        File file = newFile(dir2, "file.txt");
        assertTrue(file.createNewFile());
        File dest = newFile("test/dest");

        IO.copyDirectory(dir, dest);

        assertTrue(dir.exists());
        assertTrue(dir2.exists());
        assertTrue(file.exists());
        assertTrue(newFile(dest.getPath() + File.separator + dir2.getName(), file.getName()).exists());
    }

    // region Utils
    private static File newFile(String parent, String path) {
        return newFile(parent + "/" + path);
    }

    private static File newFile(File parent, String path) {
        return newFile(parent.getPath() + "/" + path);
    }

    private static File newFile(String path) {
        return new File(path.replace("/", File.separator).replace("\\", File.separator));
    }
    // endregion

}
