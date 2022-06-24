package net.insprill.customcarmanager.ui.elements;

import javafx.fxml.FXML;
import javafx.scene.Parent;
import javafx.scene.control.Button;
import javafx.scene.layout.Pane;
import javafx.scene.text.Text;
import net.insprill.customcarmanager.cars.Car;
import net.insprill.customcarmanager.cars.CarManager;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.ui.Window;
import net.insprill.customcarmanager.ui.dialog.ConfirmationDialog;
import net.insprill.customcarmanager.ui.dialog.InfoDialog;
import net.insprill.customcarmanager.ui.factory.FXMLFactory;

import java.io.IOException;

/**
 * A UI element representing a CCL car.
 */
public class CarElement {

    private final Parent parent;
    private final Controller controller;

    public CarElement(Car car) throws IOException {
        FXMLFactory.FXMLElement<Controller> fxml = FXMLFactory.load("/ui/car.fxml");
        this.parent = fxml.parent();
        this.controller = fxml.controller();
        this.controller.car_name.setText(car.getName());
        this.controller.delete_button.setText(Locale.getLine("window.cars.delete-button"));
    }

    /**
     * @return The {@link Parent} of this element.
     */
    public Parent getParent() {
        return this.parent;
    }

    /**
     * Toggles the background color between a lighter and darker color.
     *
     * @param toggle The background toggle.
     */
    public void toggleBackgroundColor(boolean toggle) {
        this.controller.car_background.setStyle("-fx-background-color: #" + ((toggle) ? "FAFAFA" : "F0F0F0"));
    }

    public static class Controller {

        public Pane car_background;
        public Text car_name;
        public Button delete_button;

        @FXML
        public void deleteCar() {
            if (!CarManager.checkInstallDir(true))
                return;

            String carName = car_name.getText();
            String confirmMsg = Locale.getLine("dialog.confirmation.delete-car").formatted(carName);
            if (!ConfirmationDialog.show(confirmMsg))
                return;

            Window.getInstance().getCarManager().getCar(carName).deleteAndUpdate();

            InfoDialog.show(Locale.getLine("dialog.info.car-deleted").formatted(carName));
        }

    }

}
