import pymysql

db = pymysql.connect(host='192.168.2.158', port=3306,
                     user='root', passwd='12345678',
                     db='litentry', charset='utf8')

cursor = db.cursor()


cursor.execute("SELECT VERSION()")


data = cursor.fetchone()

print('version is {}'.format(data))

db.close()
