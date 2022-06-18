package net.insprill.customcarmanager.cars;

import net.insprill.customcarmanager.config.Config;

import java.io.File;
import java.util.ArrayList;
import java.util.List;

public class CarManager {

    private static final String CARS_DIR = "Mods" + File.separator + "DVCustomCarLoader" + File.separator + "Cars";

    private final List<Car> cars = new ArrayList<>();

    public void findCars() {
        File dvDir = new File(Config.getString("install-directory"));
        File carsDir = new File(dvDir, CARS_DIR);
        for (File file : carsDir.listFiles()) {
            if (!file.isDirectory())
                continue;
            this.cars.add(new Car(file));
        }
    }

    public void installCarFromFolder(File file) {

    }

    public void installCarFromArchive(File file) {

    }

}
