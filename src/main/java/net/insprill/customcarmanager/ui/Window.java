package net.insprill.customcarmanager.ui;

import javafx.application.Application;
import javafx.application.Platform;
import javafx.fxml.FXML;
import javafx.scene.Parent;
import javafx.scene.Scene;
import javafx.scene.control.Button;
import javafx.scene.control.TextField;
import javafx.scene.image.Image;
import javafx.scene.input.DragEvent;
import javafx.scene.input.TransferMode;
import javafx.scene.layout.VBox;
import javafx.scene.text.Text;
import javafx.stage.FileChooser;
import javafx.stage.Stage;
import net.insprill.customcarmanager.cars.Car;
import net.insprill.customcarmanager.cars.CarManager;
import net.insprill.customcarmanager.config.Config;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.ui.dialog.ErrorDialog;
import net.insprill.customcarmanager.ui.elements.CarElement;
import net.insprill.customcarmanager.ui.factory.FXMLFactory;
import net.insprill.customcarmanager.ui.factory.FileChooserFactory;
import net.insprill.customcarmanager.ui.factory.FolderChooserFactory;

import java.io.File;
import java.io.IOException;
import java.util.List;

public final class Window extends Application {

    public static final String CUSTOM_CAR_LOADER_HOME_PAGE = "https://www.nexusmods.com/derailvalley/mods/324";

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
    private Controller controller;

    @Override
    public void start(Stage primaryStage) throws IOException {
        Window.setInstance(this);

        this.primaryStage = primaryStage;
        this.carManager = new CarManager();

        primaryStage.setTitle(Locale.getLine("window.title"));
        primaryStage.getIcons().add(new Image(getClass().getClassLoader().getResourceAsStream("icons/icon.png")));

        FXMLFactory.FXMLElement<Controller> fxml = FXMLFactory.load("/ui/home.fxml");

        Parent root = fxml.parent();

        this.controller = fxml.controller();

        this.controller.install_dir_header.setText(Locale.getLine("window.install-dir.header"));
        this.controller.install_dir_field.setText(Config.getString("install-directory"));
        this.controller.select_install_dir_button.setText(Locale.getLine("window.install-dir.button"));
        this.controller.install_car_folder_button.setText(Locale.getLine("window.cars.install-from-folder"));
        this.controller.install_car_archive_button.setText(Locale.getLine("window.cars.install-from-archive"));
        this.controller.installed_cars_header.setText(Locale.getLine("window.cars.installed.header"));

        Scene scene = new Scene(root, 600, 400);

        primaryStage.setScene(scene);
        primaryStage.setResizable(false);
        primaryStage.show();

        root.requestFocus();

        if (!CarManager.checkInstallDir(false)) {
            this.controller.setInstallDir(""); // Reset if the user did something weird in the config file.
            return;
        }

        getCarManager().updateCars();
    }

    /**
     * @return The primary {@link Stage} of the application.
     */
    public Stage getPrimaryStage() {
        return this.primaryStage;
    }

    /**
     * @return The {@link CarManager} associated with the application.
     */
    public CarManager getCarManager() {
        return this.carManager;
    }

    /**
     * Regenerates the car list from the {@link CarManager}'s cache.
     */
    public void populateCarList() {
        this.controller.car_list.getChildren().clear();
        boolean alternate = false;
        try {
            for (Car car : getCarManager().getCars()) {
                CarElement element = new CarElement(car);
                element.toggleBackgroundColor(alternate);
                this.controller.car_list.getChildren().add(element.getParent());
                alternate = !alternate;
            }
        } catch (IOException e) {
            ErrorDialog.show(e);
        }
    }

    public static class Controller {

        public Button install_car_archive_button;
        public Button install_car_folder_button;
        public Text install_dir_header;
        public Text installed_cars_header;
        public VBox car_list;
        public Button select_install_dir_button;
        public TextField install_dir_field;

        @FXML
        private void selectInstallDirectory() {
            File file = FolderChooserFactory.newDialog(Locale.getLine("folder-chooser.dv-install-directory.title"));
            if (file == null)
                return;

            if (!CarManager.checkInstall(file.getAbsolutePath())) {
                setInstallDir(""); // Reset if the user did something weird in the config file.
                return;
            }

            setInstallDir(file.getAbsolutePath());
            Window.getInstance().getCarManager().updateCars();
        }

        /**
         * Sets the config's {@code install-directory} field, and updates the install_dir_field {@link TextField}.
         *
         * @param dir The directory to set.
         */
        private void setInstallDir(String dir) {
            Config.setString("install-directory", dir);
            install_dir_field.setText(dir);
        }

        @FXML
        private void onDragOver(DragEvent event) {
            if (!event.getDragboard().hasFiles())
                return;

            event.acceptTransferModes(TransferMode.COPY_OR_MOVE);
            event.consume();
        }

        @FXML
        private void onDragDropped(DragEvent event) {
            if (!CarManager.checkInstallDir(true))
                return;
            if (!event.getDragboard().hasFiles())
                return;

            event.setDropCompleted(true);
            event.consume();

            List<File> files = event.getDragboard().getFiles();

            // If we don't run this later the drag/drop icon will stay until all dialogs are closed
            Platform.runLater(() -> {
                for (File file : files) {
                    if (file.isDirectory()) {
                        Window.getInstance().getCarManager().installCarsFromFolder(file);
                    } else {
                        Window.getInstance().getCarManager().installCarsFromArchive(file);
                    }
                }
                Window.getInstance().getCarManager().updateCars();
            });
        }

        @FXML
        private void installCarFromFolder() {
            if (!CarManager.checkInstallDir(true))
                return;

            File file = FolderChooserFactory.newDialog(Locale.getLine("folder-chooser.install-car.title"));
            if (file == null)
                return;

            Window.getInstance().getCarManager().installCarsFromFolder(file);
            Window.getInstance().getCarManager().updateCars();
        }

        @FXML
        private void installCarFromArchive() {
            if (!CarManager.checkInstallDir(true))
                return;

            File file = FileChooserFactory.newDialog(Locale.getLine("folder-chooser.install-car.title"), new FileChooser.ExtensionFilter("Archive", "*.zip", "*.ZIP", "*.rar", "*.RAR"));
            if (file == null)
                return;

            Window.getInstance().getCarManager().installCarsFromArchive(file);
            Window.getInstance().getCarManager().updateCars();
        }

    }

}
