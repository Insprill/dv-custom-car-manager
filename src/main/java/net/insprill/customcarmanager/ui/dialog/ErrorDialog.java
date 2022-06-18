package net.insprill.customcarmanager.ui.dialog;

import javafx.event.ActionEvent;
import javafx.scene.control.Alert;
import javafx.scene.control.ButtonBar;
import javafx.scene.control.ButtonType;
import javafx.scene.control.TextArea;
import javafx.stage.StageStyle;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.util.TextHelper;

public class ErrorDialog extends Alert {

    public ErrorDialog(String errorMessage) {
        super(AlertType.ERROR, null, ButtonType.CLOSE);
        init();
        setContentText(errorMessage);
        showAndWait();
    }

    public ErrorDialog(Exception ex) {
        super(AlertType.ERROR, null, ButtonType.CLOSE);
        init();

        String errorMessage = TextHelper.getStacktrace(ex);

        ButtonType copyToClipboard = new ButtonType(Locale.getLine("dialog.error.copy-stacktrace-to-clipboard"), ButtonBar.ButtonData.BACK_PREVIOUS);
        getDialogPane().getButtonTypes().add(0, copyToClipboard);

        TextArea errorText = new TextArea(errorMessage);
        errorText.setEditable(false);

        getDialogPane().lookupButton(copyToClipboard).addEventFilter(ActionEvent.ACTION, e -> {
            TextHelper.copyToClipboard(errorMessage);
            e.consume();
        });

        showAndWait();
    }

    private void init() {
        initStyle(StageStyle.UTILITY);
        headerTextProperty().setValue(Locale.getLine("dialog.error.title"));
    }

}
