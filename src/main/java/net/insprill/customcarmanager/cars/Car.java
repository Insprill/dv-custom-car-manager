package net.insprill.customcarmanager.cars;

import com.google.gson.JsonParser;
import net.insprill.customcarmanager.ui.dialog.ErrorDialog;
import net.insprill.customcarmanager.util.IO;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;

public class Car {

    private final String name;
    private final File directory;
    private final File carConfigFile;

    public Car(File directory) {
        this.directory = directory;
        this.carConfigFile = new File(directory, "car.json");
        this.name = calculateName();
    }

    private String calculateName() {
        if (carConfigFile.exists()) {
            try {
                return JsonParser.parseString(Files.readString(carConfigFile.toPath())).getAsJsonObject().get("identifier").getAsString();
            } catch (Exception e) {
                new ErrorDialog(e);
            }
        }
        return directory.getName();
    }

    public String getName() {
        return this.name;
    }

    public void delete() {
        try {
            IO.deleteDirectory(directory);
        } catch (IOException e) {
            new ErrorDialog(e);
        }
    }

}
