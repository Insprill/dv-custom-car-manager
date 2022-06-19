package net.insprill.customcarmanager.ui.dialog;

import javafx.event.ActionEvent;
import javafx.scene.control.Alert;
import javafx.scene.control.ButtonBar;
import javafx.scene.control.ButtonType;
import javafx.scene.control.TextArea;
import javafx.scene.layout.Region;
import javafx.scene.layout.VBox;
import javafx.scene.text.Text;
import javafx.stage.StageStyle;
import net.insprill.customcarmanager.config.Locale;
import net.insprill.customcarmanager.util.TextHelper;

public class ErrorDialog extends Alert {

    private final VBox content = new VBox();

    public ErrorDialog(String message) {
        this(message, null);
    }

    public ErrorDialog(Throwable ex) {
        this(null, ex);
    }

    public ErrorDialog(String message, Throwable ex) {
        super(AlertType.ERROR, null, ButtonType.CLOSE);
        init();

        if (message != null) {
            if (ex != null) {
                setMessage(message);
            } else {
                setContentText(message);
            }
        }

        if (ex != null) {
            if (message != null) {
                Region spacer = new Region();
                spacer.setPrefHeight(10);
                content.getChildren().add(spacer);
            }
            setException(ex);
            getDialogPane().setContent(content);
        }

        showAndWait();
    }

    private void setMessage(String message) {
        content.getChildren().add(new Text(message));
    }

    private void setException(Throwable ex) {
        String errorMessage = TextHelper.getStacktrace(ex);

        ButtonType copyToClipboard = new ButtonType(Locale.getLine("dialog.error.copy-stacktrace-to-clipboard"), ButtonBar.ButtonData.BACK_PREVIOUS);
        getDialogPane().getButtonTypes().add(0, copyToClipboard);

        TextArea errorText = new TextArea(errorMessage);
        errorText.setEditable(false);

        getDialogPane().lookupButton(copyToClipboard).addEventFilter(ActionEvent.ACTION, e -> {
            TextHelper.copyToClipboard(errorMessage);
            e.consume();
        });

        content.getChildren().add(errorText);
    }

    private void init() {
        initStyle(StageStyle.UTILITY);
        headerTextProperty().setValue(Locale.getLine("dialog.error.title"));
    }

}
