package net.insprill.customcarmanager.ui;

import javafx.event.ActionEvent;
import javafx.fxml.FXML;
import javafx.scene.Node;
import javafx.scene.control.TextField;
import javafx.scene.text.Text;
import javafx.stage.FileChooser;
import net.insprill.customcarmanager.config.Config;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.ui.factory.FileChooserFactory;
import net.insprill.customcarmanager.ui.factory.FolderChooserFactory;

import java.io.File;

public class UIController {

    @FXML
    private void selectInstallDirectory() {
        File file = FolderChooserFactory.newDialog(Locale.getLine("folder-chooser.dv-install-directory.title"));
        if (file == null)
            return;
        String path = file.getAbsolutePath();
        Config.setString("install-directory", path);
        TextField lookup = (TextField) Window.getInstance().findNode("#install_dir_field");
        lookup.setText(path);
        updateCars();
    }

    @FXML
    private void installCarFromFolder() {
        File file = FolderChooserFactory.newDialog(Locale.getLine("folder-chooser.install-car.title"));
        if (file == null)
            return;
        Window.getInstance().getCarManager().installCarFromFolder(file);
        updateCars();
    }

    @FXML
    private void installCarFromArchive() {
        File file = FileChooserFactory.newDialog(Locale.getLine("folder-chooser.install-car.title"), new FileChooser.ExtensionFilter("Archive", "*.zip"));
        if (file == null)
            return;
        Window.getInstance().getCarManager().installCarFromArchive(file);
        updateCars();
    }

    @FXML
    public void deleteCar(ActionEvent value) {
        String carName = ((Text) ((Node) value.getSource()).getParent().lookup("#car_name")).getText();
        Window.getInstance().getCarManager().getCar(carName).delete();
        updateCars();
    }

    private void updateCars() {
        Window.getInstance().getCarManager().findCars();
        Window.getInstance().populateCarList();
    }

}
