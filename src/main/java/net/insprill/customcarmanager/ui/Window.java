package net.insprill.customcarmanager.ui;

import javafx.application.Application;
import javafx.fxml.FXMLLoader;
import javafx.scene.Node;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.control.Labeled;
import javafx.scene.control.TextField;
import javafx.scene.image.Image;
import javafx.scene.layout.VBox;
import javafx.scene.text.Text;
import javafx.stage.Stage;
import net.insprill.customcarmanager.cars.Car;
import net.insprill.customcarmanager.cars.CarManager;
import net.insprill.customcarmanager.config.Config;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.ui.dialog.ErrorDialog;

import java.io.IOException;

public final class Window extends Application {

    // region Singleton
    private static Window instance;

    public static Window getInstance() {
        return instance;
    }

    private static void setInstance(Window instance) {
        Window.instance = instance;
    }
    // endregion

    private Stage primaryStage;
    private CarManager carManager;

    @Override
    public void start(Stage primaryStage) throws IOException {
        Window.setInstance(this);

        this.primaryStage = primaryStage;
        this.carManager = new CarManager();

        primaryStage.setTitle(Locale.getLine("window.title"));
        primaryStage.getIcons().add(new Image(getClass().getClassLoader().getResourceAsStream("icons/icon.png")));

        Parent root = FXMLLoader.load(getClass().getClassLoader().getResource("ui/home.fxml"));

        ((Text) root.lookup("#install_dir_header")).setText(Locale.getLine("window.install-dir.header"));
        ((TextField) root.lookup("#install_dir_field")).setText(Config.getString("install-directory"));
        ((Labeled) root.lookup("#select_install_dir_button")).setText(Locale.getLine("window.install-dir.button"));
        ((Labeled) root.lookup("#install_car_folder_button")).setText(Locale.getLine("window.cars.install-from-folder"));
        ((Labeled) root.lookup("#install_car_archive_button")).setText(Locale.getLine("window.cars.install-from-archive"));
        ((Text) root.lookup("#installed_cars_header")).setText(Locale.getLine("window.cars.installed.header"));

        Scene scene = new Scene(root, 600, 400);

        primaryStage.setScene(scene);
        primaryStage.setResizable(false);
        primaryStage.show();

        root.requestFocus();

        if (!CarManager.checkInstallDir(false))
            return;

        UIController.updateCars();
    }

    public Stage getPrimaryStage() {
        return this.primaryStage;
    }

    public CarManager getCarManager() {
        return this.carManager;
    }

    public Node findNode(String id) {
        return getPrimaryStage().getScene().lookup(id.startsWith("#") ? id : "#" + id);
    }

    public void populateCarList() {
        VBox carList = (VBox) findNode("#car_list");
        carList.getChildren().clear();
        boolean alternate = false;
        try {
            for (Car car : getCarManager().getCars()) {
                CarElement element = new CarElement(car);
                element.toggleBackgroundColor(alternate);
                carList.getChildren().add(element.getParent());
                alternate = !alternate;
            }
        } catch (IOException e) {
            new ErrorDialog(e);
        }
    }

}
