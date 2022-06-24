package net.insprill.customcarmanager.ui;

import javafx.fxml.FXMLLoader;
import javafx.scene.Parent;
import javafx.scene.control.Labeled;
import javafx.scene.text.Text;
import net.insprill.customcarmanager.cars.Car;
import net.insprill.customcarmanager.config.Locale;

import java.io.IOException;

public class CarElement {

    private final Parent element;

    public CarElement(Car car) throws IOException {
        element = FXMLLoader.load(getClass().getClassLoader().getResource("ui/car.fxml"));
        ((Text) element.lookup("#car_name")).setText(car.getName());
        ((Labeled) element.lookup("#delete_button")).setText(Locale.getLine("window.cars.delete-button"));
    }

    public Parent getParent() {
        return this.element;
    }

    public void toggleBackgroundColor(boolean toggle) {
        this.element.lookup("#car_background").setStyle("-fx-background-color: #" + ((toggle) ? "FAFAFA" : "F5F5F5"));
    }

}
