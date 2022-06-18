package net.insprill.customcarmanager.cars;

import net.insprill.customcarmanager.config.Config;

import java.io.File;
import java.util.ArrayList;
import java.util.List;

public class CarManager {

    private static final String CAR_CONFIG = "car.json";
    private static final String CARS_DIR = "Mods" + File.separator + "DVCustomCarLoader" + File.separator + "Cars";

    private final List<Car> cars = new ArrayList<>();

    private File getCarsDir() {
        return new File(Config.getString("install-directory"), CARS_DIR);
    }

    public void findCars() {
        for (File file : getCarsDir().listFiles()) {
            if (!file.isDirectory())
                continue;
            this.cars.add(new Car(file));
        }
    }

    public List<Car> getCars() {
        return new ArrayList<>(this.cars);
    }

    public void installCarFromFolder(File file) {

    }

    public void installCarFromArchive(File file) {

    }

}
