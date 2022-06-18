package net.insprill.customcarmanager.cars;

import net.insprill.customcarmanager.config.Config;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.ui.dialog.ErrorDialog;
import net.insprill.customcarmanager.util.IO;
import net.lingala.zip4j.ZipFile;
import net.lingala.zip4j.exception.ZipException;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class CarManager {

    private static final String CAR_CONFIG = "car.json";
    public static final String CARS_DIR = "Mods" + File.separator + "DVCustomCarLoader" + File.separator + "Cars";

    private final List<Car> cars = new ArrayList<>();

    private File getCarsDir() {
        return new File(Config.getString("install-directory"), CARS_DIR);
    }

    public void findCars() {
        this.cars.clear();
        for (File file : getCarsDir().listFiles()) {
            if (!file.isDirectory())
                continue;
            this.cars.add(new Car(file));
        }
    }

    public List<Car> getCars() {
        return new ArrayList<>(this.cars);
    }

    public Car getCar(String name) {
        for (Car car : this.cars) {
            if (car.getName().equals(name))
                return car;
        }
        return null;
    }

    public void installCarFromFolder(File file) {
        if (!file.isDirectory())
            throw new IllegalArgumentException("File must be a directory");
        file = findBaseFolder(file);
        if (file == null)
            return;

        File installDir = new File(getCarsDir(), file.getName());

        if (installDir.exists()) {
            File config = findConfig(installDir);
            if (config == null) {
                new ErrorDialog(Locale.getLine("dialog.error.dir-already-exists"));
                return;
            }
            new Car(installDir).delete();
        }

        try {
            IO.copyDirectory(file, installDir);
        } catch (IOException e) {
            new ErrorDialog(e);
            try {
                IO.deleteDirectory(installDir);
            } catch (IOException ex) {
                new ErrorDialog(e);
            }
        }
    }

    public void installCarFromArchive(File file) {
        File tempFolder;
        try {
            tempFolder = Files.createTempDirectory("dvcustomcarloader").toFile();
        } catch (IOException e) {
            new ErrorDialog(e);
            return;
        }

        try (ZipFile zipFile = new ZipFile(file)) {
            zipFile.extractAll(tempFolder.getAbsolutePath());
            installCarFromFolder(tempFolder);
        } catch (IOException e) {
            new ErrorDialog(e);
        } finally {
            try {
                IO.deleteDirectory(tempFolder);
            } catch (IOException e) {
                new ErrorDialog(e);
            }
        }
    }

    private File findBaseFolder(File file) {
        if (!file.isDirectory())
            throw new IllegalArgumentException("File must be a directory");

        File[] files = file.listFiles();

        if (findConfig(file) != null)
            return file;

        for (File subFile : files) {
            if (subFile.isDirectory()) {
                File baseFolder = findBaseFolder(subFile);
                if (baseFolder != null)
                    return baseFolder;
            }
        }

        new ErrorDialog(Locale.getLine("dialog.error.car-not-found"));
        return null;
    }

    private File findConfig(File dir) {
        return Arrays.stream(dir.listFiles()).filter(f -> f.getName().equals(CAR_CONFIG)).findFirst().orElse(null);
    }

}
