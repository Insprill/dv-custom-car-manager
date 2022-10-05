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

import java.awt.Desktop;
import java.io.File;
import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.Comparator;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ThreadLocalRandom;

public class CarManager {

    private static final String CAR_CONFIG = "car.json";
    private static final String CARS_DIR = "Mods" + File.separator + "DVCustomCarLoader" + File.separator + "Cars";
    private static final Set<String> COMPATIBLE_ARCHIVES = Set.of(".zip", ".rar");

    private final List<Car> cars = new ArrayList<>();

    /**
     * @return The directory cars are installed in.
     */
    private File getCarsDir() {
        return new File(Config.getString("install-directory"), CARS_DIR);
    }

    /**
     * Re-scans the cars directory and updates the caches and UI accordingly.
     */
    public void updateCars() {
        this.cars.clear();
        for (File file : getCarsDir().listFiles()) {
            if (!file.isDirectory())
                continue;
            this.cars.add(new Car(file));
        }
        this.cars.sort(Comparator.comparing(Car::getName));
        Window.getInstance().populateCarList();
    }

    /**
     * @return A clone of the car list.
     */
    public List<Car> getCars() {
        return new ArrayList<>(this.cars);
    }

    /**
     * Gets a car by its name.
     *
     * @param name The name of the car.
     * @return The car.
     */
    public Car getCar(String name) {
        for (Car car : this.cars) {
            if (car.getName().equals(name))
                return car;
        }
        return null;
    }

    /**
     * Installs cars from a folder, recursively.
     *
     * @param dir The directory to install the cars from.
     */
    public void installCarsFromFolder(File dir) {
        if (!dir.isDirectory())
            throw new IllegalArgumentException("File must be a directory");

        List<File> foundCars = findCars(dir);
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
                new Car(installDir).deleteAndUpdate();
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

    /**
     * Installs cars from an archive ({@code .zip} or {@code .rar}), recursively.
     *
     * @param file The archive file to install the cars from.
     */
    public void installCarsFromArchive(File file) {
        File tempFolder;
        try {
            tempFolder = Files.createTempDirectory("customcarmanager-" + ThreadLocalRandom.current().nextInt(Integer.MAX_VALUE)).toFile();
        } catch (IOException e) {
            ErrorDialog.show(Locale.getLine("dialog.error.temp-dir-creation-failed"), e);
            return;
        }

        if (!isCompatibleArchive(file)) {
            ErrorDialog.show(Locale.getLine("dialog.error.invalid-archive"));
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
            installCarsFromFolder(tempFolder);
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

    /**
     * Finds all cars in a directory, recursively.
     *
     * @param dir The directory to scan.
     * @return A list of all cars found, or an empty list if none were found.
     */
    private List<File> findCars(File dir) {
        if (!dir.isDirectory())
            throw new IllegalArgumentException("File must be a directory");

        if (findConfig(dir) != null)
            return Collections.singletonList(dir);

        File[] files = dir.listFiles();
        List<File> foundCars = new ArrayList<>();
        for (File subFile : files) {
            if (subFile.isDirectory()) {
                foundCars.addAll(findCars(subFile));
            }
        }
        return foundCars;
    }

    /**
     * Finds the {@code config.json} file of a car.
     *
     * @param dir The directory of the car.
     * @return The config file, or null if none was found.
     */
    private File findConfig(File dir) {
        return Arrays.stream(dir.listFiles()).filter(f -> f.getName().equals(CAR_CONFIG)).findFirst().orElse(null);
    }

    /**
     * Checks whether the provided file is a supported archive.
     *
     * @param file The file to check.
     * @return True if it's a supported archive, false otherwise.
     */
    public static boolean isCompatibleArchive(File file) {
        if (file.isDirectory())
            return false;

        int idx = file.getName().lastIndexOf('.');
        if (idx == -1)
            return false;

        return COMPATIBLE_ARCHIVES.contains(file.getName().substring(idx).toLowerCase());
    }

    /**
     * Checks that the installation directory is set, along with what {@link #checkInstall(String)} checks.
     *
     * @param error Whether the popup message show if the installation directory is not set should be an error, or an info message.
     * @return True if the installation directory is set and {@link #checkInstall(String)}'s requirements are met, false otherwise.
     */
    public static boolean checkInstallDir(boolean error) {
        String installDir = Config.getString("install-directory");

        if (installDir.isEmpty()) {
            String str = Locale.getLine("dialog.error.no-install-dir");
            if (error) {
                ErrorDialog.show(str);
            } else {
                InfoDialog.show(str);
            }
            return false;
        }

        return checkInstall(installDir);
    }

    /**
     * Checks that the installation directory passed in is valid, and Custom Car Loader is installed, and shows errors dialogs if not.
     *
     * @param installDir The path to the installation directory to check.
     * @return True if the installation directory is set and CCL is installed, false otherwise.
     */
    public static boolean checkInstall(String installDir) {
        File installDirFile = new File(installDir);

        if (Arrays.stream(installDirFile.listFiles()).noneMatch(f -> f.getName().equals("DerailValley.exe"))) {
            ErrorDialog.show(Locale.getLine("dialog.error.invalid-install-dir"));
            return false;
        }

        if (!new File(installDirFile, CarManager.CARS_DIR).exists()) {
            ErrorDialog.show(Locale.getLine("dialog.error.ccl-not-found"));
            try {
                Desktop.getDesktop().browse(new URI(Window.CUSTOM_CAR_LOADER_HOME_PAGE));
            } catch (IOException | URISyntaxException e) {
                ErrorDialog.show(e);
            }
            return false;
        }

        return true;
    }

}
