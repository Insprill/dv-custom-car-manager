package net.insprill.customcarmanager.ui.dialog;

import javafx.scene.control.Alert;
import javafx.scene.control.ButtonType;
import javafx.stage.StageStyle;

public class InfoDialog extends Alert {

    public InfoDialog(String message) {
        super(AlertType.INFORMATION, null, ButtonType.CLOSE);
        init();

        setContentText(message);

        showAndWait();
    }

    private void init() {
        initStyle(StageStyle.UTILITY);
        headerTextProperty().setValue(null);
    }

}
