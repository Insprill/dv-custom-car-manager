package net.insprill.customcarmanager.cars;

import com.github.junrar.Junrar;
import com.github.junrar.exception.RarException;
import net.insprill.customcarmanager.config.Config;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.ui.Window;
import net.insprill.customcarmanager.ui.dialog.ErrorDialog;
import net.insprill.customcarmanager.ui.dialog.InfoDialog;
import net.insprill.customcarmanager.util.IO;
import net.lingala.zip4j.ZipFile;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class CarManager {

    private static final String CAR_CONFIG = "car.json";
    public static final String CARS_DIR = "Mods" + File.separator + "DVCustomCarLoader" + File.separator + "Cars";

    private final List<Car> cars = new ArrayList<>();

    private File getCarsDir() {
        return new File(Config.getString("install-directory"), CARS_DIR);
    }

    public void updateCars() {
        this.cars.clear();
        for (File file : getCarsDir().listFiles()) {
            if (!file.isDirectory())
                continue;
            this.cars.add(new Car(file));
        }
        Window.getInstance().populateCarList();
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

        List<File> foundCars = findCars(file);
        if (foundCars.isEmpty()) {
            ErrorDialog.show(Locale.getLine("dialog.error.car-not-found"));
            return;
        }

        for (File car : foundCars) {
            File installDir = new File(getCarsDir(), car.getName());

            boolean updated = false;

            if (installDir.exists()) {
                File config = findConfig(installDir);
                if (config == null) {
                    ErrorDialog.show(Locale.getLine("dialog.error.dir-already-exists"));
                    continue;
                }
                new Car(installDir).delete();
                updated = true;
            }

            try {
                IO.copyDirectory(car, installDir);
                InfoDialog.show(Locale.getLine((updated) ? "dialog.info.car-updated" : "dialog.info.car-installed").formatted(car.getName()));
            } catch (IOException e) {
                ErrorDialog.show(Locale.getLine("dialog.error.install-copy-failed"), e);
                try {
                    // Ensure a possibly incomplete installation is removed
                    IO.deleteDirectory(installDir);
                } catch (IOException ex) {
                    ErrorDialog.show(e);
                }
            }
        }
    }

    public void installCarFromArchive(File file) {
        File tempFolder;
        try {
            tempFolder = Files.createTempDirectory("customcarmanager-" + ThreadLocalRandom.current().nextInt(Integer.MAX_VALUE)).toFile();
        } catch (IOException e) {
            ErrorDialog.show(Locale.getLine("dialog.error.temp-dir-creation-failed"), e);
            return;
        }

        try {
            if (file.getName().endsWith(".zip") || file.getName().endsWith(".ZIP")) {
                try (ZipFile zipFile = new ZipFile(file)) {
                    zipFile.extractAll(tempFolder.getAbsolutePath());
                }
            } else {
                Junrar.extract(file, tempFolder);
            }
            installCarFromFolder(tempFolder);
        } catch (IOException | RarException e) {
            ErrorDialog.show(Locale.getLine("dialog.error.archive-extraction-failed").formatted(file.getName()), e);
        } finally {
            try {
                IO.deleteDirectory(tempFolder);
            } catch (IOException e) {
                ErrorDialog.show(Locale.getLine("dialog.error.temp-dir-deletion-failed").formatted(tempFolder.getAbsolutePath()), e);
            }
        }
    }

    private List<File> findCars(File file) {
        if (!file.isDirectory())
            throw new IllegalArgumentException("File must be a directory");

        if (findConfig(file) != null)
            return Collections.singletonList(file);

        File[] files = file.listFiles();
        List<File> foundCars = new ArrayList<>();
        for (File subFile : files) {
            if (subFile.isDirectory()) {
                foundCars.addAll(findCars(subFile));
            }
        }
        return foundCars;
    }

    private File findConfig(File dir) {
        return Arrays.stream(dir.listFiles()).filter(f -> f.getName().equals(CAR_CONFIG)).findFirst().orElse(null);
    }

    public static boolean checkInstallDir(boolean error) {
        if (!Config.getString("install-directory").isEmpty())
            return true;

        String str = Locale.getLine("dialog.error.no-install-dir");
        if (error) {
            ErrorDialog.show(str);
        } else {
            InfoDialog.show(str);
        }

        return false;
    }

}
