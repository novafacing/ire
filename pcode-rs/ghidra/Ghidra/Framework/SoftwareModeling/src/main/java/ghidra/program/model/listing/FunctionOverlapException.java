/* ###
 * IP: GHIDRA
 * REVIEWED: YES
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * 
 *      http://www.apache.org/licenses/LICENSE-2.0
 * 
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
package ghidra.program.model.listing;

import ghidra.util.exception.UsrException;

/**
 * <CODE>FunctionOverlapException</CODE> is thrown in cases where
 * a function creation or change would result in overlapping functions.
 */
public class FunctionOverlapException extends UsrException {

	/**
	 * Constructor
	 */
    public FunctionOverlapException() {
        super("Function overlaps another.");
    }

	/**
	 * Constructor
	 * @param msg detailed message
	 */
    public FunctionOverlapException(String msg) {
        super(msg);
    }
}
