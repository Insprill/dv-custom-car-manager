package net.insprill.customcarmanager.cars;

import com.google.gson.JsonParser;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.ui.Window;
import net.insprill.customcarmanager.ui.dialog.ErrorDialog;
import net.insprill.customcarmanager.util.IO;

import java.io.File;
import java.nio.file.Files;

/**
 * A Class representing a CCL car.
 */
public class Car {

    private final String name;
    private final File directory;
    private final File carConfigFile;

    /**
     * @param directory The directory of the car.
     */
    public Car(File directory) {
        this.directory = directory;
        this.carConfigFile = new File(directory, "car.json");
        this.name = findName();
    }

    /**
     * @return Finds the name of the car from it's {@code car.json} file, or the folder name if an error occurs.
     */
    private String findName() {
        try {
            return JsonParser.parseString(Files.readString(carConfigFile.toPath())).getAsJsonObject().get("identifier").getAsString();
        } catch (Exception e) {
            ErrorDialog.show(Locale.getLine("dialog.error.car-name-not-found").formatted(directory.getAbsolutePath()), e);
            return directory.getName();
        }
    }

    /**
     * @return The name of the car.
     */
    public String getName() {
        return this.name;
    }

    public File getDirectory() {
        return this.directory;
    }

    /**
     * Moves the car to the recycle bin, and updates the UI.
     */
    public void deleteAndUpdate() {
        IO.moveToTrash(directory);
        Window.getInstance().getCarManager().updateCars();
    }

}
