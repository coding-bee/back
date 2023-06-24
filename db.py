from flask import Flask, request, jsonify

app = Flask(__name__)
db = {}


@app.route('/get', methods=['GET'])
def get():
    return jsonify(db)


@app.route('/insert', methods=['POST'])
def insert():
    data = request.get_json()
    key = data['key']
    value = data['value']
    db[key] = value
    return jsonify({'message': 'Row inserted successfully.'})


@app.route('/delete', methods=['POST'])
def delete():
    data = request.get_json()
    key = data['key']
    if key in db:
        del db[key]
        return jsonify({'message': 'Row deleted successfully.'})
    else:
        return jsonify({'error': 'Key not found.'})


if __name__ == '__main__':
    app.run()
